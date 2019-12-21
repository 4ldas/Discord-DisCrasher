package main

import (
	"bytes"
	"crypto/rand"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"github.com/fatih/color"
	"net/http"
)

var token string = "your token"

func main() {
	var gr_id string
	fmt.Print("Group ID: ")
	fmt.Scan(&gr_id)
	for {
		gcrename(token, gr_id, randStr(3))
		go spam(gr_id)
	}
}

func spam(gr_id string) {
	gcrename(token, gr_id, randStr(4))
}

func gcrename(token string, gID string, gcname string) {
	type payload struct {
		Name string `json:"name"`
	}

	body := &payload{
		Name: gcname,
	}
	buf := new(bytes.Buffer)
	_ = json.NewEncoder(buf).Encode(body)
	req, err := http.NewRequest("PATCH", "https://discordapp.com/api/v6/channels/"+gID, buf)
	if err != nil {
		fmt.Println(err)
	}
	req.Header.Add("authorization", token)
	req.Header.Add("Content-Type", "application/json")

	client := http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		fmt.Println(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode == 200 {
		color.Green(("Group name has changed"))
	}
}

func randStr(len int) string {
	buff := make([]byte, len)
	_, _ = rand.Read(buff)
	str := base64.StdEncoding.EncodeToString(buff)
	// Base 64 can be longer than len
	return str[:len]
}
