use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub enum PlatformBsdEnum {
    DragonFlyBsd,
    FreeBsd,
    NetBsd,
    OpenBsd,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub enum PlatformEnum {
    Android,
    Bsd(PlatformBsdEnum),
    ChromeOs,
    ChromecastOs,
    IOs(PlatformIOsEnum),
    Linux(PlatformLinuxEnum),
    Mac,
    NintendoSwitch,
    PlayStation,
    Unknown,
    Windows,
    Xbox,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub enum PlatformIOsEnum {
    IPad,
    IPhone,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub enum PlatformLinuxEnum {
    Arch,
    Debian,
    Fedora,
    Gentoo,
    OpenSuse,
    RedHat,
    Ubuntu,
    Unknown,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct UserPlatform(PlatformEnum);

impl UserPlatform {
    pub fn from(value: PlatformEnum) -> Self {
        Self(value)
    }

    // TODO: Improve user agent parser
    pub fn from_user_agent(user_agent_request_header: &str) -> Self {
        let priority_ordered_keywords = [
            ("Nintendo Switch", PlatformEnum::NintendoSwitch),
            ("PlayStation", PlatformEnum::PlayStation),
            ("Playstation", PlatformEnum::PlayStation),
            ("Xbox", PlatformEnum::Xbox),
            ("FreeBSD", PlatformEnum::Bsd(PlatformBsdEnum::FreeBsd)),
            ("OpenBSD", PlatformEnum::Bsd(PlatformBsdEnum::OpenBsd)),
            ("NetBSD", PlatformEnum::Bsd(PlatformBsdEnum::NetBsd)),
            ("DragonFly", PlatformEnum::Bsd(PlatformBsdEnum::DragonFlyBsd)),
            ("CrKey", PlatformEnum::ChromecastOs),
            ("Android", PlatformEnum::Android),
            ("CrOS", PlatformEnum::ChromeOs),
            ("Red Hat", PlatformEnum::Linux(PlatformLinuxEnum::RedHat)),
            ("Fedora", PlatformEnum::Linux(PlatformLinuxEnum::Fedora)),
            ("Ubuntu", PlatformEnum::Linux(PlatformLinuxEnum::Ubuntu)),
            ("Debian", PlatformEnum::Linux(PlatformLinuxEnum::Debian)),
            ("openSUSE", PlatformEnum::Linux(PlatformLinuxEnum::OpenSuse)),
            ("Gentoo", PlatformEnum::Linux(PlatformLinuxEnum::Gentoo)),
            ("Arch Linux", PlatformEnum::Linux(PlatformLinuxEnum::Arch)),
            ("Linux", PlatformEnum::Linux(PlatformLinuxEnum::Unknown)),
            ("LINUX", PlatformEnum::Linux(PlatformLinuxEnum::Unknown)),
            ("iPhone", PlatformEnum::IOs(PlatformIOsEnum::IPhone)),
            ("iphone", PlatformEnum::IOs(PlatformIOsEnum::IPhone)),
            ("iPad", PlatformEnum::IOs(PlatformIOsEnum::IPad)),
            ("ipad", PlatformEnum::IOs(PlatformIOsEnum::IPad)),
            ("Mac OS", PlatformEnum::Mac),
            ("Macintosh", PlatformEnum::Mac),
            ("Windows", PlatformEnum::Windows),
        ];

        for (keyword, enum_value) in priority_ordered_keywords {
            if user_agent_request_header.contains(keyword) {
                return Self(enum_value);
            }
        }

        Self(PlatformEnum::Unknown)
    }

    pub fn value(&self) -> &PlatformEnum {
        &self.0
    }
}
