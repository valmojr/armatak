package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"path"
	"strings"

	"github.com/indig0fox/a3go/a3interface"
	"github.com/indig0fox/a3go/assemblyfinder"
	"github.com/joho/godotenv"
)

var modulePath string = assemblyfinder.GetModulePath()
var modulePathDir string = path.Dir(modulePath)

var EXTENSION_NAME = "ARMATAK"

var a3ErrorChannel chan []string = make(chan []string)

func main() {
	fmt.Scanln()
}

func init() {
	godotenv.Load(".env")

	a3interface.SetVersion("0.0.0")
	a3interface.RegisterErrorChan(a3ErrorChannel)

	a3interface.NewRegistration("ping").
		SetDefaultResponse("[Received PING Command, starting background process]").
		SetRunInBackground(true).
		SetFunction(PingCommand).
		SetArgsFunction(PingCommandArgs).
		Register()
}

func PingCommand(
	ctx a3interface.ArmaExtensionContext,
	data string,
) (string, error) {
	dataSlice := strings.Split(data, "|")
	dataSliceWithoutPrefix := dataSlice[1:]
	for i, v := range dataSliceWithoutPrefix {
		dataSliceWithoutPrefix[i] = a3interface.RemoveEscapeQuotes(v)
	}

	s := fmt.Sprintf(ctx.SteamID + ` called the ping command to use the dll inside ` + ctx.ServerName + modulePathDir)

	pingDiscord(s)
	return s, nil
}

func PingCommandArgs(
	ctx a3interface.ArmaExtensionContext,
	command string,
	args []string,
) (string, error) {
	for i, v := range args {
		args[i] = a3interface.RemoveEscapeQuotes(v)
	}

	pingDiscord(strings.Join(args, " || "))

	return fmt.Sprintf(`["Called by %s", %q, %q]`,
		ctx.SteamID,
		command,
		args,
	), nil
}

func pingDiscord(content string) {
	webhookURL := os.Getenv("DISCORD_WEBHOOK")

	payload := Payload{
		Content:  content,
		Username: "ARMATAK",
	}

	jsonData, err := json.Marshal(payload)
	if err != nil {
		fmt.Println("Error marshalling payload:", err)
		return
	}

	req, err := http.Post(webhookURL, "application/json", bytes.NewReader(jsonData))
	if err != nil {
		fmt.Println("Error creating request:", err)
		return
	}

	defer req.Body.Close()

	if req.StatusCode != http.StatusOK {
		fmt.Println("Error sending request:", req.Status)
		return
	}
}
