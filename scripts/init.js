// Select Database
db = db.getSiblingDB("uniliste");

// Create Collections 
db.createCollection("students");
db.createCollection("teachers");
db.createCollection("resources");
db.createCollection("secretaries");

