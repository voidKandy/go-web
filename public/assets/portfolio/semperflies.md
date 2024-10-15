---
title: Semperflies Foundation
subtitle: Contracted Lead Developer
---

# Overview
**Semperflies Foundation** is a veteran owned 501c(3)
The website provides information about the foundation's board members, support resources for veterans, blog posts covering events the foundation hosts, dedications to lost veterans, testimonials from veterans who have interacted with the foundation and a merchandise page for exchanging merchandise for donations. It also includes an admin page to make it easy for the founder to add and remove any of the elements as he wishes.
The website for **Semperflies Foundation** is not yet finished, so it is not live.


# Tech Stack
The website's back-end was written in `Rust` while the front-end was written in `HTMX`, with `AplineJS` for scripting and `assemblerCSS` for easy styling. 
The site is deployed as a VPS and stores blogs, resources, dedications, testimonials and merchandise all in `postgreSQL` database, which is live on the host machine.

# My Role
As the only developer working for the company, I did everything.

### Backend
I designed and implemented the entire back-end infrastructure, including:
* Deployment of the site as a VPS
* Securing the VPS by using firewalls, authentication, etc.
* Writing the server using the `axum` web framework in Rust.
* Dockerizing the entire website using `docker-compose` for easy deployment
* Manually re-encoding any images uploaded via the admin form into `webp` to optimize for load speed and space usage


### Frontend
I designed and built out the whole front-end, including:
* Designing an easy interface that anyone can understand quickly
* Adding CSS transitions to page loads for a smoother experience
* Adding scripting where necessary using `AlpineJs`

