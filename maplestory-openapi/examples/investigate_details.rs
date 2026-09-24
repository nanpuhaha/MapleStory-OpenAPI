// examples/investigate_details.rs
use maplestory_openapi::MapleStoryClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();
    let _ = dotenvy::from_filename("../.env");

    let client = MapleStoryClient::from_env()?;

    let ocid = client.get_ocid("온앤온").await?;
    println!("OCID for 온앤온: {}", ocid);

    // 1. Raw Cash Item Equipment JSON
    let cash = client.get_character_cashitem_equipment(&ocid, None).await?;
    println!("\n=== Cash Equipment Analysis ===");
    println!("preset_no: {:?}", cash.preset_no);

    println!("\n--- Base Cash Items ---");
    if let Some(items) = &cash.cash_item_equipment_base {
        for it in items {
            println!(
                "Slot: {:?}, Name: {:?}, Icon: {:?}",
                it.cash_item_equipment_slot, it.cash_item_name, it.cash_item_icon
            );
            if it.cash_item_coloring_prism.is_some()
                || it.cash_item_coloring_prism_flag.as_deref() == Some("true")
            {
                println!(
                    "  -> Prism Flag: {:?}, Prism: {:?}",
                    it.cash_item_coloring_prism_flag, it.cash_item_coloring_prism
                );
            }
        }
    }

    println!("\n--- Preset 1 Cash Items ---");
    if let Some(items) = &cash.cash_item_equipment_preset_1 {
        for it in items {
            println!(
                "Slot: {:?}, Name: {:?}, Icon: {:?}",
                it.cash_item_equipment_slot, it.cash_item_name, it.cash_item_icon
            );
            if it.cash_item_coloring_prism.is_some()
                || it.cash_item_coloring_prism_flag.as_deref() == Some("true")
            {
                println!(
                    "  -> Prism Flag: {:?}, Prism: {:?}",
                    it.cash_item_coloring_prism_flag, it.cash_item_coloring_prism
                );
            }
        }
    }

    // 2. Beauty Equipment
    let beauty = client.get_character_beauty_equipment(&ocid, None).await?;
    println!("\n=== Beauty Equipment ===");
    println!("Hair: {:?}", beauty.character_hair);
    println!("Face: {:?}", beauty.character_face);
    println!("Skin: {:?}", beauty.character_skin);
    println!("Additional Hair: {:?}", beauty.additional_character_hair);
    println!("Additional Face: {:?}", beauty.additional_character_face);
    println!("Additional Skin: {:?}", beauty.additional_character_skin);

    // 3. Normal Equipment (Secondary Weapon & Gloves)
    let item_eq = client.get_character_item_equipment(&ocid, None).await?;
    println!("\n=== Sub Weapon & Gloves in Normal Equipment ===");
    if let Some(items) = &item_eq.item_equipment {
        for it in items {
            if it.item_equipment_slot.as_deref() == Some("보조무기")
                || it.item_equipment_slot.as_deref() == Some("장갑")
            {
                println!(
                    "Slot: {:?}, Part: {:?}, Name: {:?}, Icon: {:?}",
                    it.item_equipment_slot, it.item_equipment_part, it.item_name, it.item_icon
                );
            }
        }
    }

    // 4. Character Basic Image URL
    let basic = client.get_character_basic(&ocid, None).await?;
    println!("\n=== Character Look Image URL (Nexon Official Render) ===");
    println!("character_image: {:?}", basic.character_image);

    Ok(())
}
