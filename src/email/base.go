package email

import (
	"fmt"
	"log"
	"net/smtp"
)

func SendEmail(from string, body string, subject string) error {
	user := "3acd3f59956324"
	password := "00c8117ea308fe"

	to := []string{
		"ezra@voidkandy.space",
	}

	addr := "sandbox.smtp.mailtrap.io:2525"
	host := "sandbox.smtp.mailtrap.io"
	msgFormat := "From: %s\r\n" +
		"To: %s\r\n" +
		"Subject: %s\r\n\r\n" +
		"%s\r\n"

	msg := []byte(fmt.Sprintf(msgFormat, from, to, subject, body))
	auth := smtp.PlainAuth("", user, password, host)
	err := smtp.SendMail(addr, auth, from, to, msg)
	if err != nil {
		log.Fatal(err)
		return err
	}

	fmt.Println("Email sent successfully")
	return nil
}
