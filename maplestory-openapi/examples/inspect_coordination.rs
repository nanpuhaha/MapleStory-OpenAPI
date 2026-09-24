// inspect_coordination.rs
use maplestory_openapi::MapleStoryClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("NEXON_API_KEY").expect("Set NEXON_API_KEY environment variable");

    let client = MapleStoryClient::new(api_key)?;

    for name in &["온앤온", "니니류"] {
        println!("\n==========================================");
        println!("  캐릭터: {}", name);
        println!("==========================================");

        let ocid = client.get_ocid(name).await?;
        println!("OCID: {}", ocid);

        let beauty = client.get_character_beauty_equipment(&ocid, None).await?;
        println!("\n[뷰티 / 외형 기본]");
        if let Some(hair) = &beauty.character_hair {
            println!(
                "  헤어: {:?} (기본색: {:?}, 믹스색: {:?}, 믹스비율: {:?})",
                hair.hair_name, hair.base_color, hair.mix_color, hair.mix_rate
            );
        }
        if let Some(face) = &beauty.character_face {
            println!(
                "  성형: {:?} (기본색: {:?}, 믹스색: {:?}, 믹스비율: {:?})",
                face.face_name, face.base_color, face.mix_color, face.mix_rate
            );
        }
        if let Some(skin) = &beauty.character_skin {
            println!("  피부: {:?}", skin.skin_name);
        }

        let cash = client.get_character_cashitem_equipment(&ocid, None).await?;
        println!("\n[캐시 장비]");
        println!("  preset_no: {:?}", cash.preset_no);
        if let Some(items) = &cash.cash_item_equipment_base {
            println!("  base 캐시 장비 ({}개):", items.len());
            for item in items {
                println!(
                    "    - slot: {:?} / part: {:?} -> {:?}",
                    item.cash_item_equipment_slot,
                    item.cash_item_equipment_part,
                    item.cash_item_name
                );
            }
        }
        if let Some(items) = &cash.cash_item_equipment_preset_1 {
            println!("  preset_1 캐시 장비 ({}개):", items.len());
            for item in items {
                println!(
                    "    - slot: {:?} / part: {:?} -> {:?}",
                    item.cash_item_equipment_slot,
                    item.cash_item_equipment_part,
                    item.cash_item_name
                );
            }
        }

        let item_eq = client.get_character_item_equipment(&ocid, None).await?;
        println!("\n[일반 장비 (외형 관련 부위)]");
        if let Some(items) = &item_eq.item_equipment {
            for item in items {
                let slot = item.item_equipment_slot.as_deref().unwrap_or("");
                // 외형에 영향을 줄 수 있는 대표 슬롯들
                if [
                    "모자",
                    "상의",
                    "하의",
                    "한벌옷",
                    "신발",
                    "장갑",
                    "망토",
                    "무기",
                    "보조무기",
                    "눈장식",
                    "얼굴장식",
                    "귀고리",
                ]
                .contains(&slot)
                {
                    println!(
                        "    - slot: {:?} / part: {:?} -> {:?} (모루외형: {:?})",
                        item.item_equipment_slot,
                        item.item_equipment_part,
                        item.item_name,
                        item.item_shape_name
                    );
                }
            }
        }
    }

    Ok(())
}
