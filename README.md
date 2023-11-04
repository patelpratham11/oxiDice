![oxiDice Logo](/imgs/oxiDice.png)

# oxiDice

A simple CLI to generate passwords as either codes or phrases with customizable features to pick a password suited to your need. 

## Features

- Pass Code
    - Alphanumeric passcodes
    - Can add/remove numbers and/or special characters
- Pass Phrase
    - Based on the concept of diceware
    - Select words from 5 dice
- Entropy 
    - Calculates the associated entropy of the password generated
    - $E = log_{2}(R^L)$
        - E $\to$ Entropy of password
        - R $\to$ Number of *unique* characters
        - L $\to$ Length of the password generated

