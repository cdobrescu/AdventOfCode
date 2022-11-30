# A (new) rusteacean Advent of Code attempt

This is my attempt at AoC 2022 edition

I currently included some utility code to sort out the authentication process in order to automate the download of exercise input data (I've could've used the session cookie in my browser but that would've been to easy)

The authentication is done using Github Oauth

## TODOs
* document the code
* add better error handling
* add caching to minimize the number of requests to AoC pages
* generate scafolding/boilerplate per exercise
    * main function that reads downloaded input data
    * test scafolding
    * submit result method (keep track of the last submission timestamp)
    

