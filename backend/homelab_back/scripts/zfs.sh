#!/usr/bin/env bash
#
# zfs.sh — provision / inspect / tear down the ZFS volume that backs the NAS (STOR-1).
#
# This is the single source of truth for the manual steps in the STOR-1 plan.
# It is IDEMPOTENT: re-running `provision` on an existing pool/dataset is safe.
#
#   ./scripts/zfs.sh provision     # create pool + dataset + delegation (needs root)
#   ./scripts/zfs.sh status        # show usage / quota / reservation  (no root)
#   ./scripts/zfs.sh destroy       # DANGER: destroy the dev pool       (needs root)
#
# Configuration (precedence: explicit env var > apps/nas-server/.env > default):
#   ZFS_POOL              pool name                         (default: pavuk-dev)
#   ZFS_DATASET           dataset name                      (default: $ZFS_POOL/nas)
#   ZFS_MOUNTPOINT        where the dataset mounts          (default: /srv/pavuk/nas)
#   ZFS_SIZE              initial quota+reservation         (default: 5G)
#   ZFS_SERVICE_USER      OS user nas-server runs as        (default: invoking user)
#   ZFS_DEVICE            real disk/partition for a PROD pool (default: unset -> file-backed)
#   ZFS_BACKING_FILE      dev vdev image path               (default: /var/lib/pavuk/zfs-dev.img)
#   ZFS_BACKING_SIZE      dev vdev image size               (default: 10G)
#   ZFS_ASHIFT            pool ashift                       (default: 12)
#
set -euo pipefail

# ---- config resolution --------------------------------------------------------

# Pull ZFS_* keys from the app's .env, but only if not already set in the environment.
ENV_FILE="${ENV_FILE:-.env}"
if [[ -f "$ENV_FILE" ]]; then
  while IFS='=' read -r key val; do
    val="${val%\"}"; val="${val#\"}"            # strip optional surrounding quotes
    [[ -z "${!key:-}" ]] && export "$key=$val"  # existing env wins
  done < <(grep -E '^ZFS_(POOL|DATASET|MOUNTPOINT)=' "$ENV_FILE" || true)
fi

POOL="${ZFS_POOL:-pavuk-dev}"
DATASET="${ZFS_DATASET:-$POOL/nas}"
MOUNTPOINT="${ZFS_MOUNTPOINT:-/srv/pavuk/nas}"
SIZE="${ZFS_SIZE:-5G}"
SERVICE_USER="${ZFS_SERVICE_USER:-${SUDO_USER:-$USER}}"
DEVICE="${ZFS_DEVICE:-}"
BACKING_FILE="${ZFS_BACKING_FILE:-/var/lib/pavuk/zfs-dev.img}"
BACKING_SIZE="${ZFS_BACKING_SIZE:-10G}"
ASHIFT="${ZFS_ASHIFT:-12}"

# ---- helpers ------------------------------------------------------------------

log()  { printf '  \033[36m›\033[0m %s\n' "$*"; }
ok()   { printf '  \033[32m✓\033[0m %s\n' "$*"; }
warn() { printf '  \033[33m!\033[0m %s\n' "$*" >&2; }
die()  { printf '  \033[31m✗ %s\033[0m\n' "$*" >&2; exit 1; }

need_root() { [[ $EUID -eq 0 ]] || die "this command needs root — run: sudo $0 $CMD"; }
has_pool()    { zpool list -H -o name "$POOL"    &>/dev/null; }
has_dataset() { zfs   list -H -o name "$DATASET" &>/dev/null; }

require_tools() {
  command -v zfs   >/dev/null || die "zfs not found — install: sudo apt install zfsutils-linux"
  command -v zpool >/dev/null || die "zpool not found — install: sudo apt install zfsutils-linux"
}

# ---- subcommands --------------------------------------------------------------

cmd_provision() {
  require_tools; need_root
  log "pool=$POOL dataset=$DATASET size=$SIZE mount=$MOUNTPOINT user=$SERVICE_USER"

  # 1. pool (idempotent)
  if has_pool; then
    ok "pool '$POOL' already exists"
  else
    if [[ -n "$DEVICE" ]]; then
      log "creating PROD pool '$POOL' on device $DEVICE"
      zpool create -o "ashift=$ASHIFT" "$POOL" "$DEVICE"
    else
      log "creating dev pool '$POOL' on file $BACKING_FILE ($BACKING_SIZE)"
      mkdir -p "$(dirname "$BACKING_FILE")"
      [[ -f "$BACKING_FILE" ]] || truncate -s "$BACKING_SIZE" "$BACKING_FILE"
      zpool create -o "ashift=$ASHIFT" "$POOL" "$BACKING_FILE"
    fi
    ok "pool created"
  fi

  # 2. dataset (idempotent)
  if has_dataset; then
    ok "dataset '$DATASET' already exists"
  else
    log "creating dataset '$DATASET' mounted at $MOUNTPOINT"
    zfs create -o "mountpoint=$MOUNTPOINT" "$DATASET"
    ok "dataset created"
  fi

  # 3. size (idempotent — set replaces, never adds)
  log "setting quota=$SIZE reservation=$SIZE"
  zfs set "quota=$SIZE" "reservation=$SIZE" "$DATASET"

  # 4. ownership + least-privilege delegation
  chown -R "$SERVICE_USER":"$SERVICE_USER" "$MOUNTPOINT" 2>/dev/null || true
  log "delegating quota,reservation to '$SERVICE_USER'"
  zfs allow "$SERVICE_USER" quota,reservation "$DATASET"

  # 5. verify the unprivileged runtime path actually works
  if sudo -u "$SERVICE_USER" zfs set "reservation=$SIZE" "$DATASET" 2>/dev/null; then
    ok "delegation verified: '$SERVICE_USER' can set quota/reservation WITHOUT sudo"
  else
    warn "delegated 'zfs set' failed as '$SERVICE_USER'. Some OpenZFS/Linux setups need a"
    warn "sudoers fallback instead. Add (visudo):"
    warn "  $SERVICE_USER ALL=(root) NOPASSWD: /usr/sbin/zfs set quota=* reservation=* $DATASET"
    warn "and have the Rust code call 'sudo zfs …'."
  fi

  echo
  ok "provisioned. Put these in .env (backend/homelab_back/.env):"
  echo "      ZFS_POOL=$POOL"
  echo "      ZFS_DATASET=$DATASET"
  echo "      ZFS_MIN_HEADROOM_BYTES=1073741824"
}

cmd_status() {
  require_tools
  has_pool    || die "pool '$POOL' does not exist — run: sudo $0 provision"
  has_dataset || die "dataset '$DATASET' does not exist — run: sudo $0 provision"
  echo "Dataset:"
  zfs list -o name,used,avail,refer,quota,reservation,mountpoint "$DATASET"
  echo; echo "Pool:"
  zpool list "$POOL"
  echo; echo "Delegated permissions:"
  zfs allow "$DATASET" || true
}

cmd_destroy() {
  require_tools; need_root
  has_pool || { ok "pool '$POOL' already gone"; return; }
  warn "This will DESTROY pool '$POOL' and ALL data in '$DATASET'."
  if [[ "${ZFS_YES:-}" != "1" ]]; then
    read -r -p "  Type the pool name '$POOL' to confirm: " reply
    [[ "$reply" == "$POOL" ]] || die "aborted"
  fi
  zpool destroy "$POOL"
  ok "pool destroyed"
  if [[ -z "$DEVICE" && -f "$BACKING_FILE" ]]; then
    rm -f "$BACKING_FILE"
    ok "removed backing file $BACKING_FILE"
  fi
}

# ---- dispatch -----------------------------------------------------------------

CMD="${1:-}"
case "$CMD" in
  provision) cmd_provision ;;
  status)    cmd_status ;;
  destroy)   cmd_destroy ;;
  *) echo "usage: $0 {provision|status|destroy}"; exit 1 ;;
esac
