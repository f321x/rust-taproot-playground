use bitcoin::{address, secp256k1::{rand, Keypair, Secp256k1}, Network, XOnlyPublicKey};

pub fn generate_address() {
	let network = Network::Signet;

	// Generator point G(x, y)
	// public_key = G(x, y) * private_key
	let curve_func = Secp256k1::new();

	// generate random point on secp256k1 curve
	let (secret_key, _public_key) = curve_func
														.generate_keypair(&mut rand::thread_rng());

	// derive keypair from schnorr algorithm
	let keypair = Keypair::from_secret_key(&curve_func, &secret_key);

	// An x-only public key, used for verification of Taproot signatures and serialized according to BIP-340.
	let x_only_pubkey = XOnlyPublicKey::from_keypair(&keypair);

	let taproot_address = address::Address::p2tr(&curve_func, x_only_pubkey.0, None, network);

	println!("Taproot address: {}", taproot_address.to_string());
}

// works, address tb1peu999v89cs4t90qv53aq8ddeqg6jgrmg254n83yyn5j37hzntx9qwclsfl now contains some coins on mutinynet :)
// https://mutinynet.com/tx/ab4b28ba3cea0cad2bb6763809e418a5845066748d8aafdcbb950e8efa65b95c
