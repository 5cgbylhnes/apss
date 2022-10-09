#! /usr/bin/env bash
FILES=$(find . -name "*.rs" -not -path "./target/*")

sed -i -r "s/G1(Affine|Projective|Compressed)/G3\1/g" $FILES
sed -i -r "s/\[u8;(\s*)96\]/\[u8; REPLACE_ME\]/g" $FILES
sed -i -r "s/G2(Affine|Projective|Compressed)/G1\1/g" $FILES
sed -i -r "s/\[u8;(\s*)48\]/\[u8; 96\]/g" $FILES
sed -i -r "s/G3(Affine|Projective|Compressed)/G2\1/g" $FILES
sed -i -r "s/\[u8; REPLACE_ME\]/\[u8; 48\]/g" $FILES
sed -i -r "s/\[u8; REPLACE_ME\]/\[u8; 48\]/g" $FILES
sed -i "s/.*SED REPLACE 1A.*/        Bls12::pairing(\&self.generators.1.to_affine(), \&commitment.sub(value).to_affine()) == Bls12::pairing(\&self.g2_tau.sub(\&self.generators.1.mul(x)).to_affine(), \&witness.to_affine())  \/\/ SED REPLACE 1C/g" $FILES 
sed -i "s/.*SED REPLACE 1B.*/        Bls12::pairing(\&commitment.sub(value).to_affine(), \&self.generators.1.to_affine()) == Bls12::pairing(\&witness.to_affine(), \&self.g2_tau.sub(\&self.generators.1.mul(x)).to_affine())  \/\/ SED REPLACE 1D/g" $FILES
sed -i "s/.*SED REPLACE 2A.*/        let res = Bls12::multi_miller_loop(\&[ (\&sig.sig, \&G2Prepared::from(-G2Affine::generator())), (\&G1Projective::hash_to_curve(msg.as_ref(), dst.as_bytes(), \&[]).to_affine(), \&G2Prepared::from(self.pk)) ]).final_exponentiation();  \/\/ SED REPLACE 2C/g" $FILES
sed -i "s/.*SED REPLACE 2B.*/        let res = Bls12::multi_miller_loop(\&[ (\&-G1Affine::generator(), \&G2Prepared::from(sig.sig)), (\&self.pk, \&G2Prepared::from(G2Projective::hash_to_curve(msg.as_ref(), dst.as_bytes(), \&[]).to_affine())) ]).final_exponentiation();  \/\/ SED REPLACE 2D/g" $FILES
sed -i -r "s/SED REPLACE (.)C/SED REPLACE \1B/g" $FILES
sed -i -r "s/SED REPLACE (.)D/SED REPLACE \1A/g" $FILES
