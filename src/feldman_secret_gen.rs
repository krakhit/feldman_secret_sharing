use ark_ff::{Field,Zero,One, field_new, BigInteger384, Fp384, BigInteger256};
use ark_ec::{group::Group,short_weierstrass_jacobian::{self, GroupProjective}};
use ark_bls12_381::{Fr as F, G1Projective as G, G1Affine as GA,Fq};
use ark_poly::{
    univariate::DensePolynomial, EvaluationDomain, GeneralEvaluationDomain, Polynomial,evaluations
};
use ark_std::{UniformRand,ops::Mul,iter};
use num_bigint::{BigUint, ToBigInt};

pub fn gen_pow_tau<F:Field>(
    max_vec_size: usize,
    tau: F,
) -> Vec<F> {
    //equivalent of secret scalar
    let gen = F::one();
    let powers_of_tau: Vec<F> = iter::successors(Some(gen), |p| Some(*p * tau))
        .take(max_vec_size)
        .collect();
    //outputs the powers of tau
    powers_of_tau
}

pub fn vec_inner_prod<F:Field>(
    vec1: Vec<F>,
    vec2: Vec<F>,
)-> F {
    assert_eq!(vec1.len(),vec2.len(),"Vectors must be of same length!");

    let mut result = F::zero();
    for _i in 0.. vec1.len() {
            result += vec1[_i]*vec2[_i];
    };
    result
}

///asserts vector equality, panics if vectors are of different length
pub fn assert_vector_eq<F:Field>(
    vec1: Vec<F>,
    vec2: Vec<F>,
) {
    assert_eq!(vec1.len(),vec2.len(),"Vectors must be of same length!");
    for _i in 0.. vec1.len() {
            assert_eq!(vec1[_i],vec2[_i],"vectors are not equal!")
    };
}

pub fn eval_poly <F:Field> (
    vec: Vec<F>,
    tau: F,
) -> F {
    let dom = vec.len();
    let powers:Vec<F> = gen_pow_tau(dom,tau);
    let evals = vec_inner_prod(vec, powers);
    evals
}

pub fn generator() -> G {
    let gen_x = Fq::new(BigInteger384([10426916804456345898, 9949436114132101607, 2116649752893656464, 2281674603733731717, 4503335027273037540, 1014192490535040458]));
    let gen_y = Fq::new(BigInteger384([4292764575607675463, 8027277022219580558, 12795195303946612026, 8949592393414167562, 11455238229945451849, 1567632138114374463])); 
    let gen_z = Fq::new(BigInteger384([13152357397402124358, 11192454412404304597, 8711262086764406652, 9586804308161134670, 4363440654760793773, 1078801221258719581]));
    let gen :G= short_weierstrass_jacobian::GroupProjective::new(gen_x, gen_y, gen_z);
    gen
}

pub fn secret_encode() -> (usize,Vec<F>){
    // let secret =  F::from(BigUint::from_bytes_be(b"CTF{SHA_rNg_iS_cA_rNg_BT_dont_BE_A_FRI_errr}"));
    //secret is stored as the zeroth coefficient a_0
    let number_of_shares:usize = 32;
    //a_0,...,a_31 in a poly of degree \sum_i a_i x^i (deg 31 poly is specified by 32 points)
    let share_poly_coeff =vec![F::new(BigInteger256([11329633224827929176, 17010003931249500884, 3796197003401252062, 1386436274382240223])), 
    F::new(BigInteger256([4566920387071559790, 9193641831554812667, 6461714874608357256, 1947332258688838395])), 
    F::new(BigInteger256([4319227352993309495, 14311337478500616155, 9116932227632537818, 4484229657189505537])), 
    F::new(BigInteger256([17841600362005448134, 2874457830776288448, 568364109885015568, 1051538807310471320])), 
    F::new(BigInteger256([2449144214666475027, 6886645673969769855, 6362540888336217030, 5671969980901955885])), 
    F::new(BigInteger256([11960220295016788485, 2673777223896473633, 3423329151048408732, 5777374237845648842])), 
    F::new(BigInteger256([5396531013189302180, 438815280100603392, 17712823916085646995, 3307861153070369331])), 
    F::new(BigInteger256([18428985128434455511, 4078941031413916122, 10733500538782014612, 6478234395817526689])), 
    F::new(BigInteger256([15498729901039586362, 15953718835419258455, 10552478071574547980, 5319500448856887221])), 
    F::new(BigInteger256([2222614078333430821, 4653896548761838529, 10424864386656779425, 5641975945634209210])), 
    F::new(BigInteger256([17361351606736898814, 3488169324107429821, 6999475452772058839, 1466833090731795035])), 
    F::new(BigInteger256([15858447655738837502, 18117504696214038782, 11272791272360319238, 4794180734438718102])), 
    F::new(BigInteger256([3190351592914576486, 10839694223868352972, 3136647456354641297, 2200403926445235629])), 
    F::new(BigInteger256([15779379130382394974, 15150716473703394051, 9178155924577679112, 3751781518954077129])), 
    F::new(BigInteger256([13544829218620191887, 17792936781900167851, 7050059574905658048, 6560077927847242959])), 
    F::new(BigInteger256([8573653600542574, 2795321323138395895, 10679240585609527864, 7123661284357198462])), 
    F::new(BigInteger256([10816403337231858313, 388530578281624010, 7122155681081891418, 2445415626870239705])), 
    F::new(BigInteger256([9710088443502246321, 11526803163890201701, 13276726050514452168, 4288489492819529759])), 
    F::new(BigInteger256([10875260462126794776, 17235880472468696303, 4318028086558937545, 509900960670307088])), 
    F::new(BigInteger256([4132702602346725527, 8440505147911635578, 1529821956394659393, 3848464612609637023])), 
    F::new(BigInteger256([17492823295111330809, 9151540526568589178, 1521133903294087899, 2945200813336518389])), 
    F::new(BigInteger256([7961265318406913181, 10386198570208231950, 4367383190666263796, 5453007203065899093])), 
    F::new(BigInteger256([16785540965615844184, 8061590639199345402, 698375381096725686, 332033066749855282])), 
    F::new(BigInteger256([16091377084585141902, 16366044562455912254, 1433016170677878575, 7987909373901129049])), 
    F::new(BigInteger256([6457600457502892856, 18243786443652277035, 18393052098420435374, 7175513180680726140])), 
    F::new(BigInteger256([12098109834946488360, 14370084848841592296, 16956755337634090120, 7235144243379417727])), 
    F::new(BigInteger256([8795669296389093717, 16221116480577881510, 4097235343539869865, 6418254774467076400])), 
    F::new(BigInteger256([14826600036427326188, 5118316735831846523, 16836041308394006088, 4168325398054646096])), 
    F::new(BigInteger256([3950179551531315583, 10693904029196862578, 16175114606382418676, 4645391717867076969])), 
    F::new(BigInteger256([758896230240526810, 7326436097598116487, 9712543673448450346, 1808208805229868459])), 
    F::new(BigInteger256([15111214030904708815, 11313656870355535650, 13744463325538136263, 1218141238946618865])), 
    F::new(BigInteger256([7659943871647481019, 8773162376132556714, 12097176403599280652, 6990203513918317471]))];
    (number_of_shares,share_poly_coeff)
}


pub fn secret_split() -> (Vec<F>,Vec<F>) {
    let (number_of_shares,share_poly_coeff) = secret_encode();
    let mut secret_shares:Vec<F> = Vec::new();
    let mut share_id:Vec<F> = Vec::new();
    let mut temp_share_id = F::one();
    //define share_id
    for _j in 0..=number_of_shares-1{
        share_id.push(temp_share_id);
        temp_share_id+= F::one();
    }
    for i in 0..=number_of_shares-1{
        secret_shares.push(eval_poly(share_poly_coeff.clone(), share_id[i]));
    }
    //x_i, f(x_i)
    (share_id,secret_shares)
}

pub fn secret_shares_user(user_id:F) -> (F,F) {
    if user_id == F::zero() || user_id >F::from(32){
       panic!("Nice try, keep looking");
    };
    let (share_id, secret_share) = secret_split();
    let mut ind:usize = 0;
    for i in 0..=31 {
        ind+=1;
        if user_id == share_id[i]{
            break
        } 
        else {
            continue
        };
    }
    let share_id_user = share_id[ind-1];
    let secret_share_user = secret_share[ind-1];
    (share_id_user,secret_share_user)
}

pub fn Broadcast_channel() -> (usize,Vec<G>,G) {
    let (number_of_shares,share_poly_coeff) = secret_encode();
    let gen:G = generator(); 
    let mut public_share:Vec<G> = Vec::new();
    for i in 0..=number_of_shares-1 {
        public_share.push(Group::mul(&gen, &share_poly_coeff[i]));
    }
    (number_of_shares,public_share,gen)
}

pub fn verify(share_id_user:F, secret_share_user:F) {
    let (number_of_shares,public_share,gen) = Broadcast_channel();
    // g . F(x_i) = g (a_0 +a_1 x_i + a_2 x_i^2+....) = (g.a_0,g.a_1,...)(1,x_i,x_i^2,...) = (A_0,A_1,A_2,....)(1,x_i,x_i^2,...)
    let LHS:G = Group::mul(&gen, &secret_share_user);
    // (1,x_i, x_i^2....)
    let mut share_evals = gen_pow_tau(number_of_shares, share_id_user);
    let mut RHS:G= G::zero();
    // A_j (1,x_j,x_j^2,...)
    for i in 0..=number_of_shares-1{
        RHS+= Group::mul(&public_share[i], &share_evals[i]);
    }
    assert_eq!(LHS,RHS);
    let to_print:BigUint = share_id_user.into();
    let print_text = BigUint::to_u32_digits(&to_print);
    println!("Secret share integrity verified for user id {:?} ", print_text);
}

#[test]
/// Check generator is a valid point, may suffer from dlog attacks
/// since it is some random point!
pub fn generator_check(){
    let gen:G = generator();
    //ijn Jacobi projective coordinates (X,Y,Z)
    let gen_x = gen.x;
    let gen_y = gen.y;
    let gen_z = gen.z;
    //sanity check, checks if it is a valid point on the curve
    // curve in affine is y^2 = x^3 +4 and we need to convert y = Y/Z^3  and x = X/Z^2
    let z2 = gen_z.square();
    let x3 = gen_x.square().mul(gen_x);
    let y2 = gen_y.square();
    let z6_inv =z2.mul(z2).mul(z2).inverse().unwrap();
    let LHS = y2.mul(z6_inv);
    let RHS = x3.mul(z6_inv) + Fq::from(4);
    assert_eq!(LHS,RHS);
}
#[test]
///check public shares
pub fn check_public_share(){
    let (number_of_shares_dummy,share_poly_coeff) = secret_encode();
    let (number_of_shares,public_share,gen) = Broadcast_channel();
        // // g (a_0,a_1...) = (A_0,A_1...)
    for i in 0..=number_of_shares-1{
        assert_eq!(Group::mul(&gen,&share_poly_coeff[i]),public_share[i]);
    }
}

#[test]
///sanity check
pub fn sanity_check(){
    let (share_id_user,secret_share_user)=secret_shares_user(F::from(2));
    let (number_of_shares,share_poly_coeff) = secret_encode();
    assert_eq!(eval_poly(share_poly_coeff, share_id_user),secret_share_user);
}

//full sharing check
#[test]
pub fn full_sharing_test(){
    for i in 0..=31{
        let (share_id_user,secret_share_user)=secret_shares_user(F::from(i+1));
        verify(share_id_user, secret_share_user);
    }
}