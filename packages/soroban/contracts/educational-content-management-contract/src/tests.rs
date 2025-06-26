use crate::{TokenizedEducationalContent, TokenizedEducationalContentClient};
use soroban_sdk::{
    testutils::{Address as AddressTrait, BytesN as _},
    vec, Address, BytesN, Env, String,
};

#[test]
fn test_publish_content() {
    let env = Env::default();
    let contract_id = env.register(TokenizedEducationalContent, ());
    let client = TokenizedEducationalContentClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let title = String::from_str(&env, "Introduction to Blockchain");
    let content_hash = BytesN::random(&env);
    let subject_tags = vec![
        &env,
        String::from_str(&env, "blockchain"),
        String::from_str(&env, "education"),
    ];

    // Configure authentication for the creator
    env.mock_all_auths();

    // Publish content
    let content_id = client.publish_content(&creator, &title, &content_hash, &subject_tags);

    // Get content and verify
    let content = client.get_content(&content_id);
    assert_eq!(content.id, content_id);
    assert_eq!(content.creator, creator);
    assert_eq!(content.title, title);
    assert_eq!(content.content_hash, content_hash);
    assert_eq!(content.subject_tags, subject_tags);
    assert_eq!(content.upvotes, 0);
    assert_eq!(content.is_verified, false);
}

#[test]
fn test_multiple_content_publish() {
    let env = Env::default();
    let contract_id = env.register(TokenizedEducationalContent, ());
    let client = TokenizedEducationalContentClient::new(&env, &contract_id);

    let creator = Address::generate(&env);

    // Configure authentication for the creator
    env.mock_all_auths();

    // Publish first content
    let title1 = String::from_str(&env, "Introduction to Smart Contracts");
    let content_hash1 = BytesN::random(&env);
    let subject_tags1 = vec![
        &env,
        String::from_str(&env, "smart contracts"),
        String::from_str(&env, "beginner"),
    ];
    let content_id1 = client.publish_content(&creator, &title1, &content_hash1, &subject_tags1);

    // Publish second content
    let title2 = String::from_str(&env, "Advanced Smart Contract Development");
    let content_hash2 = BytesN::random(&env);
    let subject_tags2 = vec![
        &env,
        String::from_str(&env, "smart contracts"),
        String::from_str(&env, "advanced"),
    ];
    let content_id2 = client.publish_content(&creator, &title2, &content_hash2, &subject_tags2);

    // Verify IDs are sequential
    assert_eq!(content_id2, content_id1 + 1);

    // Verify both contents can be retrieved
    let content1 = client.get_content(&content_id1);
    let content2 = client.get_content(&content_id2);

    assert_eq!(content1.title, title1);
    assert_eq!(content2.title, title2);
}

#[test]
fn test_publish_empty_tags() {
    let env = Env::default();
    let contract_id = env.register(TokenizedEducationalContent, ());
    let client = TokenizedEducationalContentClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let title = String::from_str(&env, "Content with no tags");
    let content_hash = BytesN::random(&env);
    let subject_tags = vec![&env]; // Empty tags vector

    // Configure authentication for the creator
    env.mock_all_auths();

    // Publish content with empty tags
    let content_id = client.publish_content(&creator, &title, &content_hash, &subject_tags);

    // Verify content was published with empty tags
    let content = client.get_content(&content_id);
    assert_eq!(content.subject_tags, subject_tags);
}

#[test]
fn test_upvote_content() {
    let env = Env::default();
    let contract_id = env.register(TokenizedEducationalContent, ());
    let client = TokenizedEducationalContentClient::new(&env, &contract_id);

    // Configure authentication for all users
    env.mock_all_auths();

    // Create content
    let creator = Address::generate(&env);
    let title = String::from_str(&env, "Blockchain Basics");
    let content_hash = BytesN::random(&env);
    let subject_tags = vec![&env, String::from_str(&env, "blockchain")];
    let content_id = client.publish_content(&creator, &title, &content_hash, &subject_tags);

    // Upvote content
    let voter1 = Address::generate(&env);
    let upvotes = client.upvote_content(&content_id, &voter1);
    assert_eq!(upvotes, 1);

    // Verify upvote was recorded
    let content = client.get_content(&content_id);
    assert_eq!(content.upvotes, 1);

    // Test second upvote from different user
    let voter2 = Address::generate(&env);
    let upvotes = client.upvote_content(&content_id, &voter2);
    assert_eq!(upvotes, 2);
}

#[test]
fn test_multiple_upvotes() {
    let env = Env::default();
    let contract_id = env.register(TokenizedEducationalContent, ());
    let client = TokenizedEducationalContentClient::new(&env, &contract_id);

    // Configure authentication for all users
    env.mock_all_auths();

    // Create content
    let creator = Address::generate(&env);
    let title = String::from_str(&env, "Web3 Development");
    let content_hash = BytesN::random(&env);
    let subject_tags = vec![
        &env,
        String::from_str(&env, "web3"),
        String::from_str(&env, "development"),
    ];
    let content_id = client.publish_content(&creator, &title, &content_hash, &subject_tags);

    // Add multiple upvotes (10 different users)
    let expected_upvotes = 10;
    for i in 0..expected_upvotes {
        let voter = Address::generate(&env);
        let upvotes = client.upvote_content(&content_id, &voter);
        assert_eq!(upvotes, i + 1);
    }

    // Verify final upvote count
    let content = client.get_content(&content_id);
    assert_eq!(content.upvotes, expected_upvotes);
}

#[test]
#[should_panic(expected = "user has already voted for this content")]
fn test_duplicate_upvote() {
    let env = Env::default();
    let contract_id = env.register(TokenizedEducationalContent, ());
    let client = TokenizedEducationalContentClient::new(&env, &contract_id);

    // Configure authentication for all users
    env.mock_all_auths();

    // Create content
    let creator = Address::generate(&env);
    let title = String::from_str(&env, "Smart Contracts 101");
    let content_hash = BytesN::random(&env);
    let subject_tags = vec![&env, String::from_str(&env, "smart contracts")];
    let content_id = client.publish_content(&creator, &title, &content_hash, &subject_tags);

    // Upvote content
    let voter = Address::generate(&env);
    client.upvote_content(&content_id, &voter);

    // Try to upvote again - should panic
    client.upvote_content(&content_id, &voter);
}

#[test]
#[should_panic(expected = "content with ID")]
fn test_upvote_nonexistent_content() {
    let env = Env::default();
    let contract_id = env.register(TokenizedEducationalContent, ());
    let client = TokenizedEducationalContentClient::new(&env, &contract_id);

    // Configure authentication
    env.mock_all_auths();

    // Try to upvote content that doesn't exist
    let voter = Address::generate(&env);
    client.upvote_content(&999, &voter);
}

#[test]
fn test_verify_content() {
    let env = Env::default();
    let contract_id = env.register(TokenizedEducationalContent, ());
    let client = TokenizedEducationalContentClient::new(&env, &contract_id);

    // Configure authentication
    env.mock_all_auths();

    // Create content
    let creator = Address::generate(&env);
    let title = String::from_str(&env, "Advanced Cryptography");
    let content_hash = BytesN::random(&env);
    let subject_tags = vec![
        &env,
        String::from_str(&env, "cryptography"),
        String::from_str(&env, "security"),
    ];
    let content_id = client.publish_content(&creator, &title, &content_hash, &subject_tags);

    // Verify the content is not verified initially
    let content = client.get_content(&content_id);
    assert_eq!(content.is_verified, false);

    // Verify content
    let verifier = Address::generate(&env);
    let verified = client.verify_content(&content_id, &verifier);
    assert_eq!(verified, true);

    // Check that the content is now verified
    let content = client.get_content(&content_id);
    assert_eq!(content.is_verified, true);
}

#[test]
fn test_multiple_verifications() {
    let env = Env::default();
    let contract_id = env.register(TokenizedEducationalContent, ());
    let client = TokenizedEducationalContentClient::new(&env, &contract_id);

    // Configure authentication
    env.mock_all_auths();

    // Create content
    let creator = Address::generate(&env);
    let title = String::from_str(&env, "Cryptocurrency Fundamentals");
    let content_hash = BytesN::random(&env);
    let subject_tags = vec![&env, String::from_str(&env, "cryptocurrency")];
    let content_id = client.publish_content(&creator, &title, &content_hash, &subject_tags);

    // First verification
    let verifier1 = Address::generate(&env);
    let verified = client.verify_content(&content_id, &verifier1);
    assert_eq!(verified, true);

    // Second verification (should not change the status)
    let verifier2 = Address::generate(&env);
    let verified = client.verify_content(&content_id, &verifier2);
    assert_eq!(verified, true);

    // The content should still be verified
    let content = client.get_content(&content_id);
    assert_eq!(content.is_verified, true);
}

#[test]
#[should_panic(expected = "content with ID")]
fn test_verify_nonexistent_content() {
    let env = Env::default();
    let contract_id = env.register(TokenizedEducationalContent, ());
    let client = TokenizedEducationalContentClient::new(&env, &contract_id);

    // Configure authentication
    env.mock_all_auths();

    // Try to verify content that doesn't exist
    let verifier = Address::generate(&env);
    client.verify_content(&999, &verifier);
}

#[test]
fn test_creator_can_verify_own_content() {
    let env = Env::default();
    let contract_id = env.register(TokenizedEducationalContent, ());
    let client = TokenizedEducationalContentClient::new(&env, &contract_id);

    // Configure authentication
    env.mock_all_auths();

    // Create content
    let creator = Address::generate(&env);
    let title = String::from_str(&env, "Self-verified Content");
    let content_hash = BytesN::random(&env);
    let subject_tags = vec![&env, String::from_str(&env, "self-verified")];
    let content_id = client.publish_content(&creator, &title, &content_hash, &subject_tags);

    // Creator verifies their own content
    client.verify_content(&content_id, &creator);

    // Check that the content is now verified
    let content = client.get_content(&content_id);
    assert_eq!(content.is_verified, true);
}

#[test]
fn test_content_with_long_title_and_many_tags() {
    let env = Env::default();
    let contract_id = env.register(TokenizedEducationalContent, ());
    let client = TokenizedEducationalContentClient::new(&env, &contract_id);

    // Configure authentication
    env.mock_all_auths();

    // Create content with long title
    let creator = Address::generate(&env);
    let long_title = String::from_str(&env, "This is a very long title for educational content that tests the storage and retrieval of lengthy metadata strings in the Soroban smart contract platform");
    let content_hash = BytesN::random(&env);

    // Create many tags
    let mut subject_tags = vec![&env];
    for i in 1..20 {
        // Create tags directly without unused temporary variable
        let full_tag = if i < 10 {
            match i {
                1 => String::from_str(&env, "tag01"),
                2 => String::from_str(&env, "tag02"),
                3 => String::from_str(&env, "tag03"),
                4 => String::from_str(&env, "tag04"),
                5 => String::from_str(&env, "tag05"),
                6 => String::from_str(&env, "tag06"),
                7 => String::from_str(&env, "tag07"),
                8 => String::from_str(&env, "tag08"),
                9 => String::from_str(&env, "tag09"),
                _ => unreachable!(),
            }
        } else {
            match i {
                10 => String::from_str(&env, "tag10"),
                11 => String::from_str(&env, "tag11"),
                12 => String::from_str(&env, "tag12"),
                13 => String::from_str(&env, "tag13"),
                14 => String::from_str(&env, "tag14"),
                15 => String::from_str(&env, "tag15"),
                16 => String::from_str(&env, "tag16"),
                17 => String::from_str(&env, "tag17"),
                18 => String::from_str(&env, "tag18"),
                19 => String::from_str(&env, "tag19"),
                _ => unreachable!(),
            }
        };

        subject_tags.push_back(full_tag);
    }

    // Publish content with long title and many tags
    let content_id = client.publish_content(&creator, &long_title, &content_hash, &subject_tags);

    // Verify content was stored correctly
    let content = client.get_content(&content_id);
    assert_eq!(content.title, long_title);
    assert_eq!(content.subject_tags, subject_tags);
    assert_eq!(content.subject_tags.len(), 19);
}

#[test]
fn test_multiple_content_and_popularity_tracking() {
    let env = Env::default();
    let contract_id = env.register(TokenizedEducationalContent, ());
    let client = TokenizedEducationalContentClient::new(&env, &contract_id);

    // Configure authentication
    env.mock_all_auths();

    // Create 5 different content entries
    let creator = Address::generate(&env);
    let mut content_ids = vec![&env];

    for i in 0..5 {
        // Replace concatenation with static strings
        let title = match i {
            0 => String::from_str(&env, "Educational Content 0"),
            1 => String::from_str(&env, "Educational Content 1"),
            2 => String::from_str(&env, "Educational Content 2"),
            3 => String::from_str(&env, "Educational Content 3"),
            4 => String::from_str(&env, "Educational Content 4"),
            _ => unreachable!(),
        };

        let content_hash = BytesN::random(&env);
        let subject_tags = vec![&env, String::from_str(&env, "education")];

        let content_id = client.publish_content(&creator, &title, &content_hash, &subject_tags);
        content_ids.push_back(content_id);
    }

    // Different voting patterns for each content:
    // Content 0: 5 votes
    // Content 1: 3 votes
    // Content 2: 0 votes
    // Content 3: 10 votes
    // Content 4: 1 vote

    // Create a pool of voters
    let mut voters = vec![&env];
    for _ in 0..15 {
        voters.push_back(Address::generate(&env));
    }

    // Content 0: 5 votes (voters 0-4)
    for i in 0..5 {
        client.upvote_content(&content_ids.get(0).unwrap(), &voters.get(i).unwrap());
    }

    // Content 1: 3 votes (voters 5-7)
    for i in 5..8 {
        client.upvote_content(&content_ids.get(1).unwrap(), &voters.get(i).unwrap());
    }

    // Content 2: 0 votes (skip)

    // Content 3: 10 votes (voters 0-9)
    for i in 0..10 {
        client.upvote_content(&content_ids.get(3).unwrap(), &voters.get(i).unwrap());
    }

    // Content 4: 1 vote (voter 10)
    client.upvote_content(&content_ids.get(4).unwrap(), &voters.get(10).unwrap());

    // Verify vote counts
    let content0 = client.get_content(&content_ids.get(0).unwrap());
    let content1 = client.get_content(&content_ids.get(1).unwrap());
    let content2 = client.get_content(&content_ids.get(2).unwrap());
    let content3 = client.get_content(&content_ids.get(3).unwrap());
    let content4 = client.get_content(&content_ids.get(4).unwrap());

    assert_eq!(content0.upvotes, 5);
    assert_eq!(content1.upvotes, 3);
    assert_eq!(content2.upvotes, 0);
    assert_eq!(content3.upvotes, 10);
    assert_eq!(content4.upvotes, 1);

    // The most popular content should be content3
    assert!(content3.upvotes > content0.upvotes);
    assert!(content3.upvotes > content1.upvotes);
    assert!(content3.upvotes > content2.upvotes);
    assert!(content3.upvotes > content4.upvotes);
}

#[test]
fn test_verify_before_and_after_upvotes() {
    let env = Env::default();
    let contract_id = env.register(TokenizedEducationalContent, ());
    let client = TokenizedEducationalContentClient::new(&env, &contract_id);

    // Configure authentication
    env.mock_all_auths();

    // Create content
    let creator = Address::generate(&env);
    let title = String::from_str(&env, "Verification and Upvotes Interaction");
    let content_hash = BytesN::random(&env);
    let subject_tags = vec![
        &env,
        String::from_str(&env, "testing"),
        String::from_str(&env, "verification"),
    ];

    let content_id = client.publish_content(&creator, &title, &content_hash, &subject_tags);

    // Scenario 1: Verify first, then upvote
    let verifier = Address::generate(&env);
    client.verify_content(&content_id, &verifier);

    // Check content is verified
    let content = client.get_content(&content_id);
    assert_eq!(content.is_verified, true);
    assert_eq!(content.upvotes, 0);

    // Now add some upvotes
    let voters = [
        Address::generate(&env),
        Address::generate(&env),
        Address::generate(&env),
    ];

    for voter in &voters {
        client.upvote_content(&content_id, voter);
    }

    // Verify upvotes were added and verification status maintained
    let content_after_votes = client.get_content(&content_id);
    assert_eq!(content_after_votes.is_verified, true);
    assert_eq!(content_after_votes.upvotes, 3);

    // Scenario 2: New content - upvote first, then verify
    let title2 = String::from_str(&env, "Upvotes before Verification");
    let content_hash2 = BytesN::random(&env);
    let content_id2 = client.publish_content(&creator, &title2, &content_hash2, &subject_tags);

    // Add upvotes first
    let voters2 = [Address::generate(&env), Address::generate(&env)];

    for voter in &voters2 {
        client.upvote_content(&content_id2, voter);
    }

    // Check content has upvotes but is not verified
    let content2 = client.get_content(&content_id2);
    assert_eq!(content2.is_verified, false);
    assert_eq!(content2.upvotes, 2);

    // Now verify the content
    let verifier2 = Address::generate(&env);
    client.verify_content(&content_id2, &verifier2);

    // Check content is now verified and upvotes remain
    let content2_after_verify = client.get_content(&content_id2);
    assert_eq!(content2_after_verify.is_verified, true);
    assert_eq!(content2_after_verify.upvotes, 2);
}

#[test]
fn test_complex_workflow() {
    let env = Env::default();
    let contract_id = env.register(TokenizedEducationalContent, ());
    let client = TokenizedEducationalContentClient::new(&env, &contract_id);

    // Configure authentication
    env.mock_all_auths();

    // 1. Create several content entries
    let creator1 = Address::generate(&env);
    let creator2 = Address::generate(&env);

    // Content 1
    let content_id1 = client.publish_content(
        &creator1,
        &String::from_str(&env, "Solidity Security"),
        &BytesN::random(&env),
        &vec![
            &env,
            String::from_str(&env, "solidity"),
            String::from_str(&env, "security"),
        ],
    );

    // Content 2
    let content_id2 = client.publish_content(
        &creator2,
        &String::from_str(&env, "Rust for Blockchain"),
        &BytesN::random(&env),
        &vec![
            &env,
            String::from_str(&env, "rust"),
            String::from_str(&env, "blockchain"),
        ],
    );

    // 2. Upvote both contents
    let voters = [
        Address::generate(&env),
        Address::generate(&env),
        Address::generate(&env),
        Address::generate(&env),
        Address::generate(&env),
    ];

    // Vote for content 1 (3 votes)
    client.upvote_content(&content_id1, &voters[0]);
    client.upvote_content(&content_id1, &voters[1]);
    client.upvote_content(&content_id1, &voters[2]);

    // Vote for content 2 (2 votes)
    client.upvote_content(&content_id2, &voters[3]);
    client.upvote_content(&content_id2, &voters[4]);

    // 3. Verify only content 2
    let verifier = Address::generate(&env);
    client.verify_content(&content_id2, &verifier);

    // 4. Retrieve and check both contents
    let content1 = client.get_content(&content_id1);
    let content2 = client.get_content(&content_id2);

    // Content 1 should have 3 upvotes and not be verified
    assert_eq!(content1.upvotes, 3);
    assert_eq!(content1.is_verified, false);

    // Content 2 should have 2 upvotes and be verified
    assert_eq!(content2.upvotes, 2);
    assert_eq!(content2.is_verified, true);
}
