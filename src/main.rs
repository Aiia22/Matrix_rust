fn main() {
    // Matrice initiale 3×3
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    // On transpose la matrice
    let transposed = transpose(matrix);

    // Affichage de la matrice initiale et de la matrice transposée
    println!("Matrice de départ : {:?}", matrix);
    println!("Matrice transposée : {:?}", transposed);
}

// Fonction qui transpose une matrice 3×3
fn transpose(m: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    // Initialisation d'une matrice 3×3 vide pour stocker le résultat
    let mut result = [[0; 3]; 3];
    //todo!(); for debug purpose.

    // Boucle sur les lignes de la matrice
    for i in 0..3 {
        // Boucle sur les colonnes de la matrice
        for j in 0..3 {
            // On assigne l'élément transposé à la position correspondante
            result[i][j] = m[j][i];
        }
    }

    // Retourne la matrice transposée
    result
}
