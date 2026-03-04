<script lang="ts">
    let isExpanded = $state(true);

    function toggleSidebar () {
        isExpanded = !isExpanded;
    }
</script>

<nav class="sidebar" class:collapsed={!isExpanded}>
    <div class="header">
        {#if isExpanded}
            <h2>Homelab</h2>
        {/if}

        <button class="toggle-btn" onclick={toggleSidebar}>
            {isExpanded ? '◀' : '▶'}
        </button>
    </div>

    <ul>
        <li id="dashboard">
            <a href="/">
                <span class="icon">🏠</span>
                <span class="label">Dashboard</span>
            </a>
        </li>
        <li>
            <a href="/nas">
                <span class="icon">📁</span>
                <span class="label">NAS Storage</span>
            </a>
        </li>
        <li>
            <a href="/profile">
                <span class="icon">👤</span>
                <span class="label">Profile</span>
            </a>
        </li>
    </ul>

</nav>

<style>
    /* Base styles (Expanded) */
    .sidebar {
        width: 200px;
        background: #1e1e2f;
        color: white;
        padding: 1rem;
        display: flex;
        flex-direction: column;
        transition: width 0.2s ease; /* Adds a smooth sliding animation! */
    }

    .sidebar.collapsed {
        width: 60px;
        padding: 1rem 0.5rem;
    }

    /* When the sidebar has the 'collapsed' class, hide the title and all labels */
    .sidebar.collapsed .brand-title,
    .sidebar.collapsed .label {
        display: none;
    }

    /* Center the icons when collapsed */
    .sidebar.collapsed a {
        justify-content: center;
    }

    /* Center the toggle button when collapsed */
    .sidebar.collapsed .header {
        justify-content: center;
    }
    /* ------------------- */

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 2rem;
        min-height: 30px;
    }

    h2 {
        margin: 0;
        font-size: 1.2rem;
    }

    .toggle-btn {
        background: none;
        border: none;
        color: white;
        cursor: pointer;
        padding: 0.2rem;
        border-radius: 4px;
    }

    .toggle-btn:hover {
        background: rgba(255, 255, 255, 0.1);
    }

    ul {
        list-style: none;
        padding: 0;
        margin: 0;
    }

    a {
        color: #cdcdcd;
        text-decoration: none;
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 0.75rem 0.5rem;
        opacity: 0.7;
        border-radius: 6px;
    }

    .sidebar.collapsed a {
        justify-content: center; /* Center icons when collapsed */
    }

    a:hover {
        color: white;
        background: rgba(255, 255, 255, 0.05);
    }

    #dashboard a {
        font-size: 1.1rem;
        opacity: 1;
    }
</style>