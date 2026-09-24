// examples/avatar_decoding_and_look.rs
//! Demonstrates parsing, decoding, and constructing MapleStory character avatar URLs.
//!
//! Run with:
//! ```bash
//! cargo run --example avatar_decoding_and_look
//! ```

use maplestory_openapi::{
    AvatarAction, AvatarCode, AvatarEmotion, AvatarLookQuery, MapleStoryClient, WeaponMotion,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    println!("=== MapleStory Character Avatar & Look Analysis ===\n");

    // 1. Example URLs provided by the user
    let openapi_look_url = "https://open.api.nexon.com/static/maplestory/character/look/JFKCLCKKAMIBFPCLOJLPFLPENKBMBCDCGBKHKEFKCHIGFAJCLGLHBKHDEKHJOFDAJJKDCGCPIHEGOBHGHMNIKBBJNLCJJBEFIKPPIJGGILKBFONKONLBIFAKBMKEHGNBPCBODNGIAENCFMCPAPNPAFDMNDILONOEFKPHLLFCEDMIGNOAOJLOJNIDHGKIKEMNJDLAIJEHJGOIDNLBAJMDPFACNNNLMENJODAINPKOGHFNLFOIKNPJDFMMOHDKKDHFMABLDDGIPODDDEMLIIKACAPBGHIHFMGFIMAPDHADNMMIMAKDAMOGJFLFOKOKADBBHCLPAOLMAKEAGFGOPPELONHOHAECNJKCFNFDBNMIADEOFNIPPPNCEKCPFBLMHDBAPJPIDDNHFMKPNLGPMLNBHFCPJJGHAODBGBLJPEAACLNCCFAPPLGPKFBEIEIONFLIIGDIDHNNNPGPMEOPOBMDAEMDNDEGADCIKANMNDDAHHJNFMJOBMDIIHOGJCMNPIKFFHDNDAGIGPLDIODKHBLHKCIGJPEBBAKADOEJNIEIEBECMMLEMFGOEELBGLNPACHNAAMAGIHMBJIAIIEBCANDOJOLJJLDGHADNHECOPDKCOGJCHJFDOEEHNMAFJAEEAGJMLOFJMAOLIKGIBAAFNICJFEMHDNCPFGBNDKJICAFJNPJCLBEDJEHGBMJIJANAFKGLEPHDLLEEFHEMAOHMAFPMONAGGBJCEIANDPPGGODBPLEFKKOOGCNBHLLNGMNFENICDJFLLFHAEJOAHPKFNHBIGPEFAFKMFFFNOKGDOHEPGBJIMEOKAFMLOGEIKADNINKJCEEHCPJMDHOJENDKEJLJKCELBFPHNMLLHBPFFHKPFELEAGCGOANNCIBCPBEIPPNDFPFKFHPMEBHDFMFJPOLLMGNIGKHDNMOHJPNMOACGKCLMAKPJHJCEDKLNPGPEGAAAKFKNNDBNLNMBPPBCIBCOEMDKANAGENBNNGKCBLFPEKIDPHCNDFPEKPJBBEEPBJM?wmotion=W00";
    let legacy_avatar_url = "https://avatar.maplestory.nexon.com/Character/180/JFKCLCKKAMIBFPCLOJLPFLPENKBMBCDCGBKHKEFKCHIGFAJCLGLHBKHDEKHJOFDAJJKDCGCPIHEGOBHGHMNIKBBJNLCJJBEFIKPPIJGGILKBFONKONLBIFAKBMKEHGNBPCBODNGIAENCFMCPAPNPAFDMNDILONOEFKPHLLFCEDMIGNOAOJLOJNIDHGKIKEMNJDLAIJEHJGOIDNLBAJMDPFACNNNLMENJODAINPKOGHFNLFOIKNPJDFMMOHDKKDHFMABLDDGIPODDDEMLIIKACAPBGHIHFMGFIMAPDHADNMMIMAKDAMOGJFLFOKOKADBBHCLPAOLMAKEAGFGOPPELONHOHAECNJKCFNFDBNMIADEOFNIPPPNCEKCPFBLMHDBAPJPIDDNHFMKPNLGPMLNBHFCPJJGHAODBGBLJPEAACLNCCFAPPLGPKFBEIEIONFLIIGDIDHNNNPGPMEOPOBMDAEMDNDEGADCIKANMNDDAHHJNFMJOBMDIIHOGJCMNPIKFFHDNDAGIGPLDIODKHBLHKCIGJPEBBAKADOEJNIEIEBECMMLEMFGOEELBGLNPACHNAAMAGIHMBJIAIIEBCANDOJOLJJLDGHADNHECOPDKCOGJCHJFDOEEHNMAFJAEEAGJMLOFJMAOLIKGIBAAFNICJFEMHDNCPFGBNDKJICAFJNPJCLBEDJEHGBMJIJANAFKGLEPHDLLEEFHEMAOHMAFPMONAGGBJCEIANDPPGGODBPLEFKKOOGCNBHLLNGMNFENICDJFLLFHAEJOAHPKFNHBIGPEFAFKMFFFNOKGDOHEPGBJIMEOKAFMLOGEIKADNINKJCEEHCPJMDHOJENDKEJLJKCELBFPHNMLLHBPFFHKPFELEAGCGOANNCIBCPBEIPPNDFPFKFHPMEBHDFMFJPOLLMGNIGKHDNMOHJPNMOACGKCLMAKPJHJCEDKLNPGPEGAAAKFKNNDBNLNMBPPBCIBCOEMDKANAGENBNNGKCBLFPEKIDPHCNDFPEKPJBBEEPBJM.png";

    // 2. Parse AvatarCode from both URLs
    let code_from_openapi = AvatarCode::from_url_or_code(openapi_look_url)?;
    let code_from_legacy = AvatarCode::from_url_or_code(legacy_avatar_url)?;

    println!("Code from OpenAPI: {} chars", code_from_openapi.len());
    println!("Code from Legacy:  {} chars", code_from_legacy.len());
    assert_eq!(
        code_from_openapi, code_from_legacy,
        "Both endpoints must parse into identical avatar codes!"
    );
    println!("✓ Both URLs resolve to the exact same 1024-character code!\n");

    // 3. Decode into raw 512 bytes (Base16 / 4-bit nibbles)
    let raw_bytes = code_from_openapi.to_bytes();
    println!("Decoded Binary Payload: {} bytes", raw_bytes.len());
    println!("First 32 bytes (hex):   {:02X?}", &raw_bytes[..32]);

    // 4. Build dynamic poses and expressions with AvatarLookQuery
    let dynamic_query = AvatarLookQuery::new()
        .action(AvatarAction::Sit)
        .emotion(AvatarEmotion::Love)
        .weapon_motion(WeaponMotion::HideWeapon)
        .size(250, 250);

    let custom_look_url = code_from_openapi.to_look_url(&dynamic_query);
    println!("\nGenerated Custom Look URL:\n  {}", custom_look_url);

    let regenerated_legacy_url = code_from_openapi.to_legacy_avatar_url(Some(180));
    println!("\nRegenerated Legacy URL:\n  {}", regenerated_legacy_url);

    // 5. If API key is present in environment, resolve full item names and coordination
    if let Ok(client) = MapleStoryClient::from_env() {
        println!("\n=== One-Click Combined Method: get_character_look_by_name ===");
        let look = client.get_character_look_by_name("온앤온").await?;
        println!(
            "Target: {} (Level {})",
            look.character_name.as_deref().unwrap_or("?"),
            look.character_level.unwrap_or(0)
        );

        println!("\n--- All Visible Items ---");
        for item in &look.visible_items {
            let prism_str = if item.has_prism() { " [Prism]" } else { "" };
            let shape_str = if item.is_anvil { " [Anvil]" } else { "" };
            let render_str = if !item.is_visually_rendered() {
                " (도트 비노출)"
            } else {
                ""
            };
            println!(
                "  - [{:6}] {:25} (캐시: {:5}){}{}{}",
                item.slot, item.name, item.is_cash, shape_str, prism_str, render_str
            );
        }

        println!("\n--- Coloring Prism Applied Items (프리즘 적용 아이템) ---");
        for item in look.prism_items() {
            let prism = item.prism().unwrap();
            println!(
                "  - [{}] {} | Hue: {:?}, Saturation: {:?}, Value: {:?}",
                item.slot, item.name, prism.hue, prism.saturation, prism.value
            );
            println!("    └ Icon URL (이미 프리즘 적용 완료): {}", item.icon.as_deref().unwrap_or("None"));
        }

        // Generate customized avatar directly from the look object
        if let Some(jump_heart_url) = look.custom_image(AvatarAction::Jump, AvatarEmotion::Love, WeaponMotion::Default, 200, 200) {
            println!("\nCustom Jump+Love Avatar from Look Object:\n  {}", jump_heart_url);
        }
    }

    Ok(())
}
