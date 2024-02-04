use serde::{Deserialize, Serialize};

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ReactionEmoji {
    /// "👍" emoji.
    #[serde(rename = "👍")]
    Like,

    /// "👎" emoji.
    #[serde(rename = "👎")]
    Dislike,

    /// "❤" emoji.
    #[serde(rename = "❤")]
    Heart,

    /// "🔥" emoji.
    #[serde(rename = "🔥")]
    Fire,

    /// "🥰" emoji.
    #[serde(rename = "🥰")]
    Love,

    /// "👏" emoji.
    #[serde(rename = "👏")]
    Clap,

    /// "😁" emoji.
    #[serde(rename = "😁")]
    Smile,

    /// "🤔" emoji.
    #[serde(rename = "🤔")]
    Think,

    /// "🤯" emoji.
    #[serde(rename = "🤯")]
    BrainExplode,

    /// "😱" emoji.
    #[serde(rename = "😱")]
    Scare,

    /// "🤬" emoji.
    #[serde(rename = "🤬")]
    Swear,

    /// "😢" emoji.
    #[serde(rename = "😢")]
    Cry,

    /// "🎉" emoji.
    #[serde(rename = "🎉")]
    Firework,

    /// "🤩" emoji.
    #[serde(rename = "🤩")]
    StarEyes,

    /// "🤮" emoji.
    #[serde(rename = "🤮")]
    Vomit,

    /// "💩" emoji.
    #[serde(rename = "💩")]
    Excrement,

    /// "🙏" emoji.
    #[serde(rename = "🙏")]
    Pray,

    /// "👌" emoji.
    #[serde(rename = "👌")]
    Ok,

    /// "🕊" emoji.
    #[serde(rename = "🕊")]
    Bird,

    /// "🤡" emoji.
    #[serde(rename = "🤡")]
    Clown,

    /// "🥱" emoji.
    #[serde(rename = "🥱")]
    Yawn,

    /// "🥴" emoji.
    #[serde(rename = "🥴")]
    Dizzy,

    /// "😍" emoji.
    #[serde(rename = "😍")]
    InLove,

    /// "🐳" emoji.
    #[serde(rename = "🐳")]
    Whale,

    /// ❤‍🔥" emoji.
    #[serde(rename = "❤‍🔥")]
    HeartInFire,

    /// "🌚" emoji.
    #[serde(rename = "🌚")]
    NewMoon,

    /// "🌭" emoji.
    #[serde(rename = "🌭")]
    Hotdog,

    /// "💯" emoji.
    #[serde(rename = "💯")]
    OneHundred,

    /// "🤣" emoji.
    #[serde(rename = "🤣")]
    Laugh,

    /// "⚡" emoji.
    #[serde(rename = "⚡")]
    Lightning,

    /// "🍌" emoji.
    #[serde(rename = "🍌")]
    Banana,

    /// "🏆" emoji.
    #[serde(rename = "🏆")]
    Goblet,

    /// "💔" emoji.
    #[serde(rename = "💔")]
    BrokenHeart,

    /// "🤨" emoji.
    #[serde(rename = "🤨")]
    Suspicion,

    /// "😐" emoji.
    #[serde(rename = "😐")]
    Apathy,

    /// "🍓" emoji.
    #[serde(rename = "🍓")]
    Strawberry,

    /// "🍾" emoji.
    #[serde(rename = "🍾")]
    Champagne,

    /// "💋" emoji.
    #[serde(rename = "💋")]
    Kiss,

    /// 🖕" emoji.
    #[serde(rename = "🖕")]
    TheFinger,

    /// "😈" emoji.
    #[serde(rename = "😈")]
    Devil,

    /// "😴" emoji.
    #[serde(rename = "😴")]
    Sleep,

    /// "😭" emoji.
    #[serde(rename = "😭")]
    Crying,

    /// "🤓" emoji.
    #[serde(rename = "🤓")]
    Nerd,

    /// "👻" emoji.
    #[serde(rename = "👻")]
    Ghost,

    /// "👨‍💻" emoji.
    #[serde(rename = "👨‍💻")]
    TechGuy,

    /// "👀" emoji.
    #[serde(rename = "👀")]
    Eyes,

    /// "🎃" emoji.
    #[serde(rename = "🎃")]
    JackOLantern,

    /// "🙈" emoji.
    #[serde(rename = "🙈")]
    MonkeyClosedEyes,

    /// "😇" emoji.
    #[serde(rename = "😇")]
    Angel,

    /// "😨" emoji.
    #[serde(rename = "😨")]
    Fear,

    /// "🤝" emoji.
    #[serde(rename = "🤝")]
    Handshake,

    /// "✍" emoji.
    #[serde(rename = "✍")]
    WritingHand,

    /// "🤗" emoji.
    #[serde(rename = "🤗")]
    Hugs,

    /// "🫡" emoji.
    #[serde(rename = "🫡")]
    Salute,

    /// "🎅" emoji.
    #[serde(rename = "🎅")]
    Santa,

    /// "🎄" emoji.
    #[serde(rename = "🎄")]
    ChristmasTree,

    /// "☃" emoji.
    #[serde(rename = "☃")]
    Snowman,

    /// "💅" emoji.
    #[serde(rename = "💅")]
    Manicure,

    /// "🤪" emoji.
    #[serde(rename = "🤪")]
    Zany,

    /// "🗿" emoji.
    #[serde(rename = "🗿")]
    Moai,

    /// "🆒" emoji.
    #[serde(rename = "🆒")]
    Cool,

    /// "💘" emoji.
    #[serde(rename = "💘")]
    Romance,

    /// "🙉" emoji.
    #[serde(rename = "🙉")]
    MonkeyOpenEyes,

    /// "🦄" emoji.
    #[serde(rename = "🦄")]
    Unicorn,

    /// "😘" emoji.
    #[serde(rename = "😘")]
    AirKiss,

    /// "💊" emoji.
    #[serde(rename = "💊")]
    RedPill,

    /// "🙊" emoji.
    #[serde(rename = "🙊")]
    MonkeyClosedMouth,

    /// "😎" emoji.
    #[serde(rename = "😎")]
    CoolFace,

    /// "👾" emoji.
    #[serde(rename = "👾")]
    Alien,

    /// "🤷‍♂" emoji.
    #[serde(rename = "🤷‍♂")]
    ManShrugging,

    /// "🤷‍♀" emoji.
    #[serde(rename = "🤷‍♀")]
    WomanShrugging,

    /// "🤷" emoji.
    #[serde(rename = "🤷")]
    Shrugging,

    /// "😡" emoji.
    #[serde(rename = "😡")]
    Anger,
}
