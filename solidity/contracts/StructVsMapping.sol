pragma solidity ^0.5.0;

/// @title Struct vs Mapping
/// @dev Is it more efficient (gas used) to use structs or mappings?
/// TODO: Test creation and calls to these contracts
/// TODO: Use libraries with delegate calls

contract StructDesign {
    struct Podcast {
        bytes32 name;
        address author;
        bool published;
    }
    mapping (uint256 => Podcast) public podcasts;

    event PodcastAdded(uint256 id);

    function addPodcast(
        uint256 id,
        bytes32 name,
        bool published,
        address author
    ) external {
        Podcast memory podcast;
        podcast.name = name;
        podcast.published = published;
        podcast.author = author;
        podcasts[id] = podcast;
        emit PodcastAdded(id);
    }
}

contract MappingDesignA {
    mapping (uint256 => bytes32) public podcastNames;
    mapping (uint256 => bool) public podcastStatuses; // naming is hard!
    mapping (uint256 => address) public podcastAuthors;

    event PodcastAdded(uint256 id);

    function addPodcast(
        uint256 id,
        bytes32 name,
        bool published,
        address author
    ) external {
        podcastNames[id] = name;
        podcastStatuses[id] = published;
        podcastAuthors[id] = author;
        emit PodcastAdded(id);
    }
}

contract MappingDesignB {
    mapping (uint256 => bytes32) podcastNames;
    mapping (uint256 => bool) podcastStatuses; // naming is hard!
    mapping (uint256 => address) podcastAuthors;

    event PodcastAdded(uint256 id);

    function addPodcast(
        uint256 id,
        bytes32 name,
        bool published,
        address author
    ) external {
        podcastNames[id] = name;
        podcastStatuses[id] = published;
        podcastAuthors[id] = author;
        emit PodcastAdded(id);
    }

    function getPodcast(uint256 id) external view returns (
        bytes32 name,
        bool published,
        address author
    ) {
        name = podcastNames[id];
        published = podcastStatuses[id];
        author = podcastAuthors[id];
    }
}
