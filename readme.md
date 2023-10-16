# Mon Projet de Restaurant

Ce projet utilise plusieurs microservices pour gérer divers aspects de l'opération d'un restaurant. Les technologies utilisées incluent MermaidJS pour la visualisation des microservices et Passport.js pour l'authentification.

## Diagramme de flux des microservices



Ce document présente une vue d'ensemble des microservices qui composent le projet. Chaque microservice est représenté graphiquement à l'aide de Mermaid.js pour une meilleure compréhension.

## Compositon Micros Services : 
```mermaid
graph LR
  subgraph "Client"
    A[Login] -->|Login Request| B[Login Service]
    A[Admin] -->|Admin Interface| F[Back Office Service]
    A[Client] -->|Menu Request| G[Menu Service]
    A[Client] -->|Subscribe| H[Newsletter Service]
    A[Client] -->|Purchase| I[Loyalty Service]
    A[Client] -->|Reservation Request| J[Reservation Service]
    A[Client] -->|Request Recipe| K[Random Recipe Service]
    A[Client] -->|Chat| L[ChatBot Service]
  end

  subgraph "Login Service"
    B -->|Authentication| C[Database]
    B -->|Authentication| D[Admin Permissions]
  end

  subgraph "Back Office Service"
    F -->|Control| C[Database]
  end

  subgraph "Menu Service"
    G -->|Retrieve Menu| F[Back Office Service]
    F -->|Data Updates| D[Database]
  end

  subgraph "Newsletter Service"
    H -->|Mailing| I[Email Provider]
  end

  subgraph "Loyalty Service"
    I -->|Point Calculation| C[Database]
  end

  subgraph "Reservation Service"
    J -->|Reservation Handling| D[Database]
  end

  subgraph "Random Recipe Service"
    K -->|Retrieve Recipe| M[Recipe Database]
  end

  subgraph "ChatBot Service"
    L -->|Conversations| N[AI Engine]
  end

```

## Login utilisateur et admin à l'aide de passport.js

```mermaid
graph TD
  subgraph "Login Service"
    A[Client] -->|Login Request| B[passport.js]
    B -->|Authentication| C[Database]
    B -->|Authentication| D[Admin Permissions]
  end
```


## Administration Back Office

```mermaid
graph TD
  subgraph "Back Office Service"
    A[Admin] -->|Admin Interface| B[Back Office]
    B -->|Control| C[Data Manipulation]
    C -->|Database| D[Database]
  end
```

## Liste des menus avec mise à jour des plats ajoutés par le Back Office

```mermaid
graph TD
  subgraph "Menu Service"
    A[Client] -->|Menu Request| B[Menu]
    B -->|Retrieve Menu| C[Back Office]
    C -->|Data Updates| D[Database]
  end
```

## Service de newsletters (mailing)
```mermaid
graph TD
  subgraph "Newsletter Service"
    A[Client] -->|Subscribe| B[Newsletter Service]
    B -->|Mailing| C[Email Provider]
  end

```

## Service de compte de fidélité (système par points)

```mermaid
graph TD
  subgraph "Loyalty Service"
    A[Client] -->|Purchase| B[Loyalty Service]
    B -->|Point Calculation| C[Database]
  end
```

## Service de réservations (pour réserver un créneau horaire et manger sur place)
```mermaid
graph TD
  subgraph "Reservation Service"
    A[Client] -->|Reservation Request| B[Reservation Service]
    B -->|Reservation Handling| C[Database]
  end
```

## Service de recette aléatoire
```mermaid
graph TD
  subgraph "Random Recipe Service"
    A[Client] -->|Request Recipe| B[Random Recipe Service]
    B -->|Retrieve Recipe| C[Recipe Database]
  end
```

## ChatBot
```mermaid
graph TD
  subgraph "ChatBot Service"
    A[Client] -->|Chat| B[ChatBot Service]
    B -->|Conversations| C[AI Engine]
  end
```

## Description de chaque microservice:

Login utilisateur et admin : Ce microservice utilise Passport.js pour gérer l'authentification des utilisateurs et des administrateurs. Il gère également la session de l'utilisateur et la persistance des données.
 
Administration Back Office : Ce microservice permet aux administrateurs de gérer les menus et les plats. Il fournit une interface pour ajouter, modifier et supprimer des éléments de menu.

Liste des menus avec mise à jour des plats ajoutés par le back office : Ce microservice gère la liste des menus et des plats disponibles. Il met à jour la liste en fonction des modifications apportées par le Back Office.

Service de newsletters (mailing) : Ce microservice gère l'envoi de newsletters aux utilisateurs. Il peut être utilisé pour envoyer des promotions, des mises à jour de menu, etc.

Service de compte de fidélité (système par points) : Ce microservice gère le système de compte de fidélité. Il attribue des points aux utilisateurs en fonction de leurs achats et des actions qu'ils effectuent sur le site.

Service de reservations (pour reserver un créneau horaire et manger sur place) : Ce microservice gère les réservations de tables. Il permet aux utilisateurs de réserver un créneau horaire et de réserver une table pour manger sur place.

Service de recette aléatoire : Ce microservice propose une recette aléatoire à chaque utilisateur. Il peut être utilisé pour encourager les utilisateurs à essayer de nouveaux plats.

ChatBot : Ce microservice fournit un chatbot pour répondre aux questions des utilisateurs et les aider à naviguer sur le site.