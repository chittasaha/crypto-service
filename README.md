# crypto-service

to run the service do the followings:

1. create an image by running the command:

    docker build -t crypto-service-image .


2. When the image is created run a container by the following command:

    docker run -p 8080:8080 crypto-service-image

3. to test open the request.http in vs code and run the first test case