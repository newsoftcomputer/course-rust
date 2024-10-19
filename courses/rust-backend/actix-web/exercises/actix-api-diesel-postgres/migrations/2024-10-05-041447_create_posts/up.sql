-- Your SQL goes here

CREATE TABLE IF NOT EXISTS public."users" (                
    id_users UUID PRIMARY KEY NOT NULL,                    
    --id_users UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    first_name CHARACTER(100),                             
    last_name CHARACTER(100),                              
    email CHARACTER(100),                                  
    status BOOLEAN NOT NULL DEFAULT FALSE                  
    -- CONSTRAINT "users" PRIMARY KEY (id_users)           
)                                                          
