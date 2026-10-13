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

func lastError(fallback string, code C.int32_t) error {
	errPtr := C.maplestory_last_error_message()
	if errPtr != nil {
		defer C.maplestory_free_string(errPtr)
		detail := C.GoString(errPtr)
		if detail != "" {
			return fmt.Errorf("%s (error code: %d): %s", fallback, int(code), detail)
		}
	}
	return fmt.Errorf("%s (error code: %d)", fallback, int(code))
}

func optionalCString(s string) (*C.char, func()) {
	if s == "" {
		return nil, func() {}
	}
	cs := C.CString(s)
	return cs, func() { C.free(unsafe.Pointer(cs)) }
}

// NewClient initializes a new client with the given API key.
func NewClient(apiKey string) (*Client, error) {
	cApiKey := C.CString(apiKey)
	defer C.free(unsafe.Pointer(cApiKey))

	handle := C.maplestory_client_new(cApiKey)
	if handle == nil {
		return nil, lastError("failed to initialize maplestory client handle", -1)
	}

	return &Client{handle: handle}, nil
}

// NewClientWithBaseURL initializes a new client with the given API key and custom base URL.
func NewClientWithBaseURL(apiKey, baseURL string) (*Client, error) {
	cApiKey := C.CString(apiKey)
	defer C.free(unsafe.Pointer(cApiKey))
	cBaseURL := C.CString(baseURL)
	defer C.free(unsafe.Pointer(cBaseURL))

	handle := C.maplestory_client_with_base_url(cApiKey, cBaseURL)
	if handle == nil {
		return nil, lastError("failed to initialize maplestory client handle", -1)
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
func (c *Client) callStringApi(fallback string, cCall func(out **C.char) C.int32_t) (string, error) {
	if c.handle == nil {
		return "", errors.New("client is closed")
	}

	var cJson *C.char
	res := cCall(&cJson)
	if res != 0 || cJson == nil {
		return "", lastError(fallback, res)
	}
	defer C.maplestory_free_string(cJson)

	return C.GoString(cJson), nil
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
	cName := C.CString(characterName)
	defer C.free(unsafe.Pointer(cName))

	return c.callStringApi("failed to get character dashboard", func(out **C.char) C.int32_t {
		return C.maplestory_get_character_dashboard(c.handle, cName, out)
	})
}

// GetGuildProfile retrieves the guild profile (guild basic + member profiles) JSON string.
// Pass an empty string for date to query the latest available data.
func (c *Client) GetGuildProfile(guildName, worldName, date string) (string, error) {
	cGuild := C.CString(guildName)
	defer C.free(unsafe.Pointer(cGuild))
	cWorld := C.CString(worldName)
	defer C.free(unsafe.Pointer(cWorld))
	cDate, freeDate := optionalCString(date)
	defer freeDate()

	return c.callStringApi("failed to get guild profile", func(out **C.char) C.int32_t {
		return C.maplestory_get_guild_profile(c.handle, cGuild, cWorld, cDate, out)
	})
}

// GetUserUnion retrieves user union level/grade information by character name as a JSON string.
func (c *Client) GetUserUnion(characterName, date string) (string, error) {
	cName := C.CString(characterName)
	defer C.free(unsafe.Pointer(cName))
	cDate, freeDate := optionalCString(date)
	defer freeDate()

	return c.callStringApi("failed to get user union", func(out **C.char) C.int32_t {
		return C.maplestory_get_user_union(c.handle, cName, cDate, out)
	})
}

// GetOverallRanking retrieves overall ranking information as a JSON string.
// Pass worldType < 0 to omit worldType filter, and page <= 0 to omit page filter.
func (c *Client) GetOverallRanking(date, worldName string, worldType int32, characterClass, ocid string, page int32) (string, error) {
	cDate := C.CString(date)
	defer C.free(unsafe.Pointer(cDate))
	cWorld, freeWorld := optionalCString(worldName)
	defer freeWorld()
	cClass, freeClass := optionalCString(characterClass)
	defer freeClass()
	cOcid, freeOcid := optionalCString(ocid)
	defer freeOcid()

	return c.callStringApi("failed to get overall ranking", func(out **C.char) C.int32_t {
		return C.maplestory_get_overall_ranking(
			c.handle,
			cDate,
			cWorld,
			C.int32_t(worldType),
			cClass,
			cOcid,
			C.int32_t(page),
			out,
		)
	})
}

// GetNoticeList retrieves the latest official notices list as a JSON string.
func (c *Client) GetNoticeList() (string, error) {
	return c.callStringApi("failed to get notice list", func(out **C.char) C.int32_t {
		return C.maplestory_get_notice_list(c.handle, out)
	})
}

// GetAccountSummary retrieves the account summary (union + artifact + champion + achievements) as a JSON string.
func (c *Client) GetAccountSummary(characterName, date string) (string, error) {
	cName := C.CString(characterName)
	defer C.free(unsafe.Pointer(cName))
	cDate, freeDate := optionalCString(date)
	defer freeDate()

	return c.callStringApi("failed to get account summary", func(out **C.char) C.int32_t {
		return C.maplestory_get_account_summary(c.handle, cName, cDate, out)
	})
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
		return nil, lastError("failed to render character GIF", res)
	}
	defer C.maplestory_free_bytes(cData, cLen)

	return C.GoBytes(unsafe.Pointer(cData), C.int(cLen)), nil
}

