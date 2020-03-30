def args = [
	channels: [
		normal: [
			success: [],
			failure: [],
			fixed: [],
			debug: ["#cp-builder"],
		],
		scheduled: [:],
		upstream: [
			success: [],
			failure: [],
			fixed: [],
			debug: ["#cp-nightly-debug"],
		],
		people: []
	],
	podTemplateMap: [
		containers: utils.setContainerTemplates(['builder':'aokservice-docker-hosted.forge.avaya.com/kp_rustbuilder:0.0.31']),
		volumes: utils.setVolumes(["docker":true, "go":false, "rust":true]),
	],
	command: '/init.sh && cargo build --target-dir /cargo/target'
]

microbuild(args)
