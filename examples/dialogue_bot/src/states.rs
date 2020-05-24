use teloxide::prelude::*;

use super::favourite_music::FavouriteMusic;
use parse_display::Display;

pub struct StartState;

pub struct ReceiveFullNameState {
    rest: StartState,
}

pub struct ReceiveAgeState {
    rest: ReceiveFullNameState,
    full_name: String,
}

pub struct ReceiveFavouriteMusicState {
    rest: ReceiveAgeState,
    age: u8,
}

#[derive(Display)]
#[display(
    "Your full name: {rest.rest.full_name}, your age: {rest.age}, your \
     favourite music: {favourite_music}"
)]
pub struct ExitState {
    rest: ReceiveFavouriteMusicState,
    favourite_music: FavouriteMusic,
}

up!(
    StartState -> ReceiveFullNameState,
    ReceiveFullNameState + [full_name: String] -> ReceiveAgeState,
    ReceiveAgeState + [age: u8] -> ReceiveFavouriteMusicState,
    ReceiveFavouriteMusicState + [favourite_music: FavouriteMusic] -> ExitState,
);

pub type Dialogue = Coprod!(
    StartState,
    ReceiveFullNameState,
    ReceiveAgeState,
    ReceiveFavouriteMusicState,
);

wrap_dialogue!(
    Wrapper(Dialogue),
    default Self(Dialogue::inject(StartState)),
);
