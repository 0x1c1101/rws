# 🤔 What is this?
It's a rust-based web scraping tool designed to extract and list significant web content.

# ✨ Features
- Extracts comment lines from HTML code snippets.
- Enumerates hyperlinks embedded within the webpage.
- Retrieves the content of the robots.txt file.
- It's fast.

# 💨 Speed

```
❯ ./rws http://127.0.0.1/ A

:::::::::  :::       :::  ::::::::
:+:    :+: :+:       :+: :+:    :+:
+:+    +:+ +:+       +:+ +:+
+#++:++#:  +#+  +:+  +#+ +#++:++#++
+#+    +#+ +#+ +#+#+ +#+        +#+
#+#    #+#  #+#+# #+#+#  #+#    #+#
###    ###   ###   ###    ########
------------------------------------
Rust Web Scraper      @heapoverfloww


robots.txt:

User-agent: GPTBot
Disallow: /

Comments:

 Favicon
---
 Google Web Fonts
---
 Font Awesome
---
 Libraries Stylesheet
---
 Customized Bootstrap Stylesheet
---
 Topbar Start
---
 Topbar End
---
 Navbar Start
---
 Navbar End
---
 Products Start
---
 Products End
---
 Offer Start
---
 Offer End
---
 Vendor Start
---
 Vendor End
---
 Footer Start
---
 Footer End
---
 Back to Top
---
 JavaScript Libraries
---
 Contact Javascript File
---
 Template Javascript
---

Hyperlinks:

img/favicon.ico
https://fonts.gstatic.com
https://fonts.googleapis.com/css2?family=Roboto:wght@400;500;700&display=swap
https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.10.0/css/all.min.css
lib/animate/animate.min.css
lib/owlcarousel/assets/owl.carousel.min.css
css/style.css
login.php
#navbar-vertical
?category=camera
?category=shirt
?category=shoes
index.html
shop.html
detail.html
cart.html
checkout.html
contact.html
/products.php?id=1
/products.php?id=2
/products.php?id=3
/products.php?id=4
/products.php?id=5
/products.php?id=6
/products.php?id=7
/products.php?id=8
/products.php?id=9
https://htmlcodex.com

It took 0.134564541 seconds!
```

# ⚙️ How to install?

```
git clone https://github.com/0x1c1101/rws
cd rws/
cargo build --release
cd target/release/
./rws
```

# 📖 How to use?

It's actually pretty simple.

```
❯ ./rws
Usage: ./rws <URL> <OPTS>
Current options:
r: Read robots.txt
c: List comment lines in the page
l: List hyperlinks in the page
A: Enable all options

Example: ./rws https://example.com rlc
```
