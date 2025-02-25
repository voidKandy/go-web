---
title: Deathwish Powersports
subtitle: Contracted Lead Developer
---

# Overview
**DeathWish Powersports** is a power-sport rental company based in the South Lake Tahoe / Carson Valley area. 
The website provides information about their company and available rental machines.
It includes a booking page with a calendar for available dates, a coupon code input field, and a form for booking and passenger information.
The contact form, accessible via a button in the bottom right corner, makes it easy to reach the **Deathwish** team.
For some nice visual flair, the background of the website has a topographical map of the area where the company is based.
Unfortunately, the website for **Deathwish Powersports** will not be live until the summer of 2025.

![medium-deathwish-logo](/public/assets/deathwish/dwps1.webp)

# Tech Stack
The website's back-end was written in `Go` while the frontend was written in `HTMX`, with some sprinklings of `_hyperscript` and `javascript` for additional reactivity. 
All the booking invoices, available rental machines, and coupon codes are stored in a `postgreSQL` database, and we used [stripe](https://stripe.com/) for all the payments.

# My Role
As the only developer working for the company, I did everything.

### Backend
I designed and implemented the entire back-end infrastructure, including:
* Integrating with stripe for secure payment processing
* Logic for invoices and coupon codes
* Ensuring automated emails with relevant booking information are sent to both customers and the company.

### Frontend
I designed and built out the whole front-end, including:
* A dynamic booking calendar that updates based on the selected machine and disables already booked dates to prevent scheduling conflicts.
* Smooth page transitions for enhanced user experience.

### Other Contributions
Beyond my developer role, I was commissioned to illustrate the company logo, shown above.
I also contributed as a brand development consultant, leveraging my experience as a musician and artistic director.
