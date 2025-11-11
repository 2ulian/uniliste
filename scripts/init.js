// Select Database
db = db.getSiblingDB("uniliste");

// Create Collection: Eleves (Students)
// Structure: _id (int - INE), nom (str), prenom (str), age (int), promo (int), groupeTD (int), groupeTP (int), groupe (str)
db.createCollection("Eleves", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["_id", "nom", "prenom", "age", "promo", "groupeTD", "groupeTP", "groupe"],
      properties: {
        _id: {
          bsonType: "int",
          description: "INE (student ID) - must be an integer and is required"
        },
        nom: {
          bsonType: "string",
          description: "Last name - must be a string and is required"
        },
        prenom: {
          bsonType: "string",
          description: "First name - must be a string and is required"
        },
        age: {
          bsonType: "int",
          description: "Age - must be an integer and is required"
        },
        promo: {
          bsonType: "int",
          description: "Promotion year - must be an integer and is required"
        },
        groupeTD: {
          bsonType: "int",
          description: "TD group number - must be an integer and is required"
        },
        groupeTP: {
          bsonType: "int",
          description: "TP group number - must be an integer and is required"
        },
        groupe: {
          bsonType: "string",
          description: "Group identifier - must be a string and is required"
        }
      }
    }
  }
});

// Create Collection: Professeurs (Professors)
// Structure: _id (ObjectId), nom (str), prenom (str), numero_ressources ([str])
db.createCollection("Professeurs", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["nom", "prenom", "numero_ressources"],
      properties: {
        nom: {
          bsonType: "string",
          description: "Last name - must be a string and is required"
        },
        prenom: {
          bsonType: "string",
          description: "First name - must be a string and is required"
        },
        numero_ressources: {
          bsonType: "array",
          items: {
            bsonType: "string"
          },
          description: "Resource numbers array - must be an array of strings and is required"
        }
      }
    }
  }
});

// Create Collection: Ressources (Resources)
// Structure: _id (str - "R1.01"), nom_de_matiere (str)
db.createCollection("Ressources", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["_id", "nom_de_matiere"],
      properties: {
        _id: {
          bsonType: "string",
          description: "Resource ID - must be a string (e.g., 'R1.01') and is required"
        },
        nom_de_matiere: {
          bsonType: "string",
          description: "Subject name - must be a string and is required"
        }
      }
    }
  }
});

// Create Collection: Secretaires (Secretaries)
// Structure: _id (ObjectId), nom (str), prenom (str)
db.createCollection("Secretaires", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["nom", "prenom"],
      properties: {
        nom: {
          bsonType: "string",
          description: "Last name - must be a string and is required"
        },
        prenom: {
          bsonType: "string",
          description: "First name - must be a string and is required"
        }
      }
    }
  }
});

