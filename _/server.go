package main

import (
	"embed"
	"github.com/gin-gonic/gin"
	"github.com/pusher/pusher-http-go/v5"
	"io"
	"log"
)

//go:embed index.html app.js
var fs embed.FS

func main() {
	client := pusherClient()
	r := gin.Default()
	r.GET("/", func(c *gin.Context) {
		index, err := fs.ReadFile("index.html")
		if err != nil {
			c.JSON(500, gin.H{
				"error": "Error reading index.html",
			})
			return
		}
		c.Data(200, "text/html", index)
	})

	r.GET("/app.js", func(c *gin.Context) {
		app, err := fs.ReadFile("app.js")
		if err != nil {
			c.JSON(500, gin.H{
				"error": "Error reading app.js",
			})
			return
		}
		c.Data(200, "text/javascript", app)
	})
	r.POST("/pusher/auth", func(c *gin.Context) {
		params, err := io.ReadAll(c.Request.Body)
		if err != nil {
			log.Printf("Error reading request body: %v", err)
			c.JSON(500, gin.H{
				"error": "Error reading request body",
			})
			return
		}

		auth, err := client.AuthorizePrivateChannel(params)
		if err != nil {
			log.Printf("Error authenticating user: %v", err)
			c.JSON(500, gin.H{
				"error": "Error authenticating user",
			})
			return
		}

		c.Data(200, "application/json", auth)
	})

	// Start server
	log.Println("Starting server on port 8080")
	if err := r.Run(":8080"); err != nil {
		panic(err)
	}
}

func pusherClient() *pusher.Client {
	return &pusher.Client{
		AppID:   "fastsocket",
		Key:     "fastsocket",
		Secret:  "secret",
		Host:    "127.0.0.1:6002",
		Secure:  false,
		Cluster: "ap1",
	}
}
