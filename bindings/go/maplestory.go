package maplestory

/*
#cgo CFLAGS: -I../c/include
#cgo LDFLAGS: -L../../target/debug -L../../target/release -lmaplestory_openapi_c
#include "maplestory_openapi.h"
#include <stdlib.h>
*/
import "C"
import (
	"errors"
	"fmt"
	"os"
	"unsafe"
)

// Client is a high-performance Go client for NEXON MapleStory OpenAPI, powered by Rust.
type Client struct {
	handle *C.ClientHandle
}

// NewClient initializes a new client with the given API key.
func NewClient(apiKey string) (*Client, error) {
	cApiKey := C.CString(apiKey)
	defer C.free(unsafe.Pointer(cApiKey))

	handle := C.maplestory_client_new(cApiKey)
	if handle == nil {
		return nil, errors.New("failed to initialize maplestory client handle")
	}

	return &Client{handle: handle}, nil
}

// FromEnv initializes a client from the NEXON_API_KEY environment variable.
func FromEnv() (*Client, error) {
	apiKey := os.Getenv("NEXON_API_KEY")
	if apiKey == "" {
		return nil, errors.New("NEXON_API_KEY environment variable is not set")
	}
	return NewClient(apiKey)
}

// Close frees the underlying native Rust client handle.
func (c *Client) Close() {
	if c.handle != nil {
		C.maplestory_client_free(c.handle)
		c.handle = nil
	}
}

// GetCharacterDashboard retrieves the comprehensive all-in-one character dashboard JSON string.
func (c *Client) GetCharacterDashboard(characterName string) (string, error) {
	if c.handle == nil {
		return "", errors.New("client is closed")
	}

	cName := C.CString(characterName)
	defer C.free(unsafe.Pointer(cName))

	var cJson *C.char
	res := C.maplestory_get_character_dashboard(c.handle, cName, &cJson)
	if res != 0 || cJson == nil {
		return "", fmt.Errorf("failed to get character dashboard (error code: %d)", int(res))
	}
	defer C.maplestory_free_string(cJson)

	return C.GoString(cJson), nil
}

// RenderCharacterGif renders an animated GIF for the character look.
// Available presets: "walk", "wink", "stand", "jump", "sit", "prone", "fly", "blink"
func (c *Client) RenderCharacterGif(characterName, preset string, width, height uint32) ([]byte, error) {
	if c.handle == nil {
		return nil, errors.New("client is closed")
	}

	cName := C.CString(characterName)
	defer C.free(unsafe.Pointer(cName))

	cPreset := C.CString(preset)
	defer C.free(unsafe.Pointer(cPreset))

	var cData *C.uint8_t
	var cLen C.size_t

	res := C.maplestory_render_character_gif(
		c.handle,
		cName,
		cPreset,
		C.uint32_t(width),
		C.uint32_t(height),
		&cData,
		&cLen,
	)

	if res != 0 || cData == nil {
		return nil, fmt.Errorf("failed to render character GIF (error code: %d)", int(res))
	}
	defer C.maplestory_free_bytes(cData, cLen)

	return C.GoBytes(unsafe.Pointer(cData), C.int(cLen)), nil
}
