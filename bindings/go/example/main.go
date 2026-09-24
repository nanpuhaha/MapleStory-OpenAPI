package main

import (
	"fmt"
	"log"
	"os"

	"github.com/yourusername/maplestory-openapi-rust/bindings/go"
)

func main() {
	client, err := maplestory.FromEnv()
	if err != nil {
		log.Fatalf("클라이언트 초기화 실패: %v", err)
	}
	defer client.Close()

	target := "온앤온"
	fmt.Printf(">> '%s' 올인원 대시보드 조회 중...\n", target)

	dashJson, err := client.GetCharacterDashboard(target)
	if err != nil {
		log.Fatalf("대시보드 조회 실패: %v", err)
	}
	fmt.Println("대시보드 JSON 획득 완료!")
	fmt.Println(dashJson[:200] + "...")

	fmt.Printf("\n>> '%s' 걷기 애니메이션 GIF 생성 중...\n", target)
	gifBytes, err := client.RenderCharacterGif(target, "walk", 128, 128)
	if err != nil {
		log.Fatalf("GIF 생성 실패: %v", err)
	}

	if err := os.WriteFile("walk_go.gif", gifBytes, 0644); err != nil {
		log.Fatalf("파일 저장 실패: %v", err)
	}
	fmt.Printf("walk_go.gif 생성 완료! (크기: %d bytes)\n", len(gifBytes))
}
