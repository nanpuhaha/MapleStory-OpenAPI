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

            let action = AvatarAction::parse_or_default(&args.action, AvatarAction::Stand1);
            let emotion = AvatarEmotion::parse_or_default(&args.emotion, AvatarEmotion::Default);
            let wmotion = WeaponMotion::parse_or_default(&args.wmotion, WeaponMotion::Default);

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
