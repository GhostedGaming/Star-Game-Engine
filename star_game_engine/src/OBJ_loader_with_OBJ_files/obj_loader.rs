pub fn load_obj() {
    //This is a model loader that only supports OBJ files.
    let mut user_input : String = "".to_string();
    std::io::stdin().read_line(&mut user_input).expect("Message Not Received");
    println!("You Have Spawned A {}",user_input);
    let cornell_box = tobj::load_obj(user_input, &tobj::GPU_LOAD_OPTIONS);
    assert!(cornell_box.is_ok());
    
    let (models,materials) = cornell_box.expect("Failed To Load OBJ File Error 2");

    // Materials might report a separate loading error if the MTL file wasn't found.
    // If you don't need the materials, you can generate a default here and use that
    // instead.
    
    let materials = materials.expect("Failed To Load MTL File");

    println!("# Of Models : {}",models.len());
    println!("# Of Materials: {}",materials.len());

    for(i,m) in models.iter().enumerate() {
        let mesh = &m.mesh;

        println!("model[{}].name = \'{}\'", i, m.name);
        println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

        println!(
            "Size Of Model[{}].face_ariteries: {}",
            i,
            mesh.face_arities.len()
        );

        let mut next_face = 0;
        for f in 0..mesh.face_arities.len() {
            let end = next_face + mesh.face_arities[f] as usize;
            let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
            println!("  face [{}] = {:?}", f,face_indices);
            next_face = end;
        }

        // Normals and texture coordinates are also loaded, but not printed in this example
        println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);

        assert!(mesh.positions.len() % 3 == 0);
        for v in 0..mesh.positions.len() / 3 {
            println!(
                "    v[{}] = ({}, {}, {})",
                v,
                mesh.positions[3 * v],
                mesh.positions[3 * v + 1],
                mesh.positions[3 * v + 2]
            );
        }
    }
    
    for (i, m) in materials.iter().enumerate() {
        println!("material[{}].name = \'{}\'", i, m.name);
        if let Some(ambient) = m.ambient {
            println!(
                "    material.Ka = ({}, {}, {})",
                ambient[0], ambient[1], ambient[2]
            );
        }
        if let Some(diffuse) = m.diffuse {
            println!(
                "    material.Kd = ({}, {}, {})",
                diffuse[0], diffuse[1], diffuse[2]
            );
        }
        if let Some(specular) = m.specular {
            println!(
                "    material.Ks = ({}, {}, {})",
                specular[0], specular[1], specular[2]
            );
        }
        if let Some(shininess) = m.shininess {
            println!("    material.Ns = {}", shininess);
        }
        if let Some(dissolve) = m.dissolve {
            println!("    material.d = {}", dissolve);
        }
        if let Some(ambient_texture) = &m.ambient_texture {
            println!("    material.map_Ka = {}", ambient_texture);
        }
        if let Some(diffuse_texture) = &m.diffuse_texture {
            println!("    material.map_Kd = {}", diffuse_texture);
        }
        if let Some(specular_texture) = &m.specular_texture {
            println!("    material.map_Ks = {}", specular_texture);
        }
        if let Some(shininess_texture) = &m.shininess_texture {
            println!("    material.map_Ns = {}", shininess_texture);
        }
        if let Some(normal_texture) = &m.normal_texture {
            println!("    material.map_Bump = {}", normal_texture);
        }
        if let Some(dissolve_texture) = &m.dissolve_texture {
            println!("    material.map_d = {}", dissolve_texture);
        }
    
        for (k, v) in &m.unknown_param {
            println!("    material.{} = {}", k, v);
        }
    }
}