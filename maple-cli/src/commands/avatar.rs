use crate::cli::{AvatarArgs, AvatarFormat};
use maplestory_openapi::avatar::{
    AnimationPreset, AvatarAction, AvatarCode, AvatarEmotion, AvatarLookQuery, WeaponMotion,
};
use maplestory_openapi::client::MapleStoryClient;
use std::error::Error;
use std::path::PathBuf;

pub async fn run(
    client: &MapleStoryClient,
    args: &AvatarArgs,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let preset = AnimationPreset::parse_or_default(&args.action, AnimationPreset::Stand);

    let (bytes, extension) = match args.format {
        AvatarFormat::Png => {
            let basic = client.character().get_by_name(&args.name).await?;
            let img_url = basic.character_image.ok_or_else(|| {
                format!("Character '{}' does not have a character image URL.", args.name)
            })?;
            let avatar_code = AvatarCode::from_url_or_code(&img_url)?;

            let action = match args.action.as_str() {
                "stand1" => AvatarAction::Stand1,
                "stand2" => AvatarAction::Stand2,
                "walk1" => AvatarAction::Walk1,
                "walk2" => AvatarAction::Walk2,
                "prone" => AvatarAction::Prone,
                "fly" => AvatarAction::Fly,
                "jump" => AvatarAction::Jump,
                "sit" => AvatarAction::Sit,
                "alert" => AvatarAction::Alert,
                other => AvatarAction::Custom(other.to_string()),
            };

            let emotion = match args.emotion.as_str() {
                "wink" => AvatarEmotion::Wink,
                "smile" => AvatarEmotion::Smile,
                "cry" => AvatarEmotion::Cry,
                "angry" => AvatarEmotion::Angry,
                "bewildered" => AvatarEmotion::Bewildered,
                "blink" => AvatarEmotion::Blink,
                "furious" => AvatarEmotion::Furious,
                "sleepy" => AvatarEmotion::Sleepy,
                "love" => AvatarEmotion::Love,
                "despair" => AvatarEmotion::Despair,
                "pain" => AvatarEmotion::Pain,
                "default" => AvatarEmotion::Default,
                other => AvatarEmotion::Custom(other.to_string()),
            };

            let wmotion = match args.wmotion.as_str() {
                "onehand" => WeaponMotion::OneHand,
                "twohand" => WeaponMotion::TwoHand,
                "gun" => WeaponMotion::Gun,
                "hideweapon" => WeaponMotion::HideWeapon,
                "default" => WeaponMotion::Default,
                other => WeaponMotion::Custom(other.to_string()),
            };

            let query = AvatarLookQuery::default()
                .action(action)
                .emotion(emotion)
                .weapon_motion(wmotion);

            let raw_bytes = client.avatar().fetch_frame_bytes(&avatar_code, &query).await?;
            (raw_bytes, "png")
        }
        AvatarFormat::Gif => {
            let gif_bytes = client
                .avatar()
                .render_gif_by_name(&args.name, &preset, None, None)
                .await?;
            (gif_bytes, "gif")
        }
        AvatarFormat::Webp => {
            let webp_bytes = client
                .avatar()
                .render_webp_by_name(&args.name, &preset, None, None)
                .await?;
            (webp_bytes, "webp")
        }
    };

    let out_path = args
        .out
        .clone()
        .unwrap_or_else(|| PathBuf::from(format!("{}.{}", args.name, extension)));

    std::fs::write(&out_path, &bytes)?;
    println!(
        "Successfully saved {} avatar to: {}",
        args.format_label(),
        out_path.display()
    );

    Ok(())
}

impl AvatarArgs {
    fn format_label(&self) -> &'static str {
        match self.format {
            AvatarFormat::Png => "PNG",
            AvatarFormat::Gif => "Animated GIF",
            AvatarFormat::Webp => "Animated WebP",
        }
    }
}
