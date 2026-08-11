use clap::ValueEnum;

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum FileTypeArg {
    Image,
    Text,
    Video,
    Audio,
    Pdf,
    Zip,
    Unknown,
}

impl FileTypeArg {
    pub fn to_proto(self) -> i32 {
        use homelab_proto::nas::FileType;
        let ft = match self {
            FileTypeArg::Image => FileType::Image,
            FileTypeArg::Text => FileType::Text,
            FileTypeArg::Video => FileType::Video,
            FileTypeArg::Audio => FileType::Audio,
            FileTypeArg::Pdf => FileType::Pdf,
            FileTypeArg::Zip => FileType::Zip,
            FileTypeArg::Unknown => FileType::Unknown,
        };
        ft as i32
    }
}
