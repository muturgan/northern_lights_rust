# northern_lights_rust
A port of my old freelance project for rust learning

### Steps for local launch:
- create a **.env** file in the root of the project and fill it out using the **env.example** pattern
- configure the data storage method:
  - start a container with a DB via command **docker-compose up** if you want the application to interact with a real database
  - or just set the environment **ENV=test** if you want the application to store all data in application memory
- run the application with the **./scripts.sh dev** (make sure you have **rust** installed, this application was tested on **1.80** rust version)
- open a browser on a **{APP_HOST}:{APP_PORT}** page to get a promo code as user
- open a browser on a **{APP_HOST}:{APP_PORT}/check** page to check and activate this promo as seller
