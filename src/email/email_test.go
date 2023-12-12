package email

import (
	"fmt"
	"log"
	"net/smtp"
	"testing"
)

func TestSendEmail(t *testing.T) {
	from := "john.doe@example.com"

	user := "3acd3f59956324"
	password := "00c8117ea308fe"

	to := []string{
		"ezra@voidkandy.space",
	}

	addr := "sandbox.smtp.mailtrap.io:2525"
	host := "sandbox.smtp.mailtrap.io"

	msg := []byte("From: john.doe@example.com\r\n" +
		"To: ezra@voidkandy.space\r\n" +
		"Subject: Test mail\r\n\r\n" +
		"Email body\r\n")

	auth := smtp.PlainAuth("", user, password, host)

	err := smtp.SendMail(addr, auth, from, to, msg)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Email sent successfully")
}
