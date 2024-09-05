
# Kuksa Data Broker :

Kuksa Data Broker is an in-vehicle data broker that provides a unified interface for data communication between various components of a vehicle. This README provides instructions to build and run the Kuksa Data Broker with the stats feature enabled using Docker.

## Prerequisites

Before you begin, ensure you have the following installed on your system:

- Docker
- Docker Compose

## Building the Kuksa Data Broker

First, you need to build the Docker image for the Kuksa Data Broker. Open your terminal and navigate to the directory containing the Dockerfile and run the following command:

#### prerequisites before building the image

refer and install: https://github.com/eclipse-kuksa/kuksa-common/tree/main/sbom-tools  
scripts: https://github.com/LikhithST/kuksa-databroker/tree/main/scripts

```sh
 export KUKSA_DATABROKER_SBOM="y"

 export KUKSA_DATABROKER_FEATURES="databroker/stats"
 
 ./scripts/build-databroker.sh amd64                # can use: arm64 amd64 riscv64

 docker buildx build --platform linux/amd64 -f scripts/Dockerfile -t kuksa-databroker . 
```

This command will build the Docker image and tag it as `kuksa-databroker`.

## Running the Kuksa Data Broker

After successfully building the Docker image, you can run the Kuksa Data Broker using Docker Compose. Execute the following command in your terminal:

```sh
docker compose up
```

Docker Compose will start the necessary containers for the Kuksa Data Broker and the services defined in your `docker-compose.yml` file.

## Stopping the Kuksa Data Broker

To stop the Kuksa Data Broker, you can use the following command:

```sh
docker compose down
```

This command will stop and remove the containers defined in the Docker Compose configuration.

## Additional Information

[Kuksa Data Broker Github Repo](https://github.com/eclipse-kuksa/kuksa-databroker).

---

By following these steps, you will be able to build and run the Kuksa Data Broker with the stats feature enabled. If you encounter any issues or have further questions, please refer to the official documentation or seek support from the community.
