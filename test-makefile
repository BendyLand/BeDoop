# === Configuration ===
TAG_FILE := .image_tag
ACCOUNT_ID ?= 767398106706
GENERATOR_IMAGE := $(ACCOUNT_ID).dkr.ecr.us-east-1.amazonaws.com/batch-generator
PROCESSOR_IMAGE := $(ACCOUNT_ID).dkr.ecr.us-east-1.amazonaws.com/batch-processor

# === Step 1: Generate and store the UUID tag ===
tag:
	@echo "🔖 Generating UUID tag..."
	@uuidgen | tee $(TAG_FILE)

# === Step 2: Build Rust binary ===
json2parquet_binary:
	@if [ ! -f json2parquet_binary ]; then \
		echo "🔧 Building json2parquet binary..."; \
		docker build -f Dockerfile.json2parquet -t json2parquet-builder . && \
		docker create --name json2parquet-temp json2parquet-builder && \
		docker cp json2parquet-temp:/app/json2parquet/target/x86_64-unknown-linux-musl/release/json2parquet ./json2parquet_binary && \
		chmod +x ./json2parquet_binary && \
		docker rm json2parquet-temp; \
	fi

# === Step 3: Build Docker images ===
build: tag json2parquet_binary
	@TAG=$$(cat $(TAG_FILE)) && \
	echo "🐳 Building generator..." && \
	docker buildx build \
	  --platform linux/amd64 \
	  --build-arg TARGETARCH=amd64 \
	  -f Dockerfile.generator -t batch-generator --load .

	@TAG=$$(cat $(TAG_FILE)) && \
	echo "🐳 Building processor..." && \
	docker buildx build \
	  --platform linux/amd64 \
	  --build-arg TARGETARCH=amd64 \
	  -f Dockerfile.processor -t batch-processor --load .

# === Step 4: Push tagged images to ECR ===
publish: build
	@TAG=$$(cat $(TAG_FILE)) && \
	echo "📤 Pushing images with tag: $$TAG" && \
	docker tag batch-generator $(GENERATOR_IMAGE):$$TAG && \
	docker tag batch-processor $(PROCESSOR_IMAGE):$$TAG && \
	docker push $(GENERATOR_IMAGE):$$TAG && \
	docker push $(PROCESSOR_IMAGE):$$TAG

# === Step 5: Deploy infrastructure via CDK ===
deploy:
	@TAG=$$(cat $(TAG_FILE)) && \
	echo "🚀 Deploying CDK with tag $$TAG" && \
	cd cdk && LAST_RUN_UUID=$$TAG ACCOUNT_ID=$(ACCOUNT_ID) cdk deploy --require-approval never

# === Step 6: Run latest ECS task ===
execute:
	@echo "🎬 Running ECS generator task..."
	@bash -c '\
		GEN_FAMILY="BatchProcessorStackBatchGeneratorTaskDefFD9C753C"; \
		GEN_TASK_DEF=$$(aws ecs list-task-definitions \
			--family-prefix $$GEN_FAMILY \
			--sort DESC \
			--max-items 1 \
			--output text \
			--query "taskDefinitionArns[0]"); \
		echo "👉 Using generator task definition: $$GEN_TASK_DEF"; \
		aws ecs run-task \
		  --cluster default \
		  --launch-type FARGATE \
		  --network-configuration '\''awsvpcConfiguration={subnets=["subnet-00939e65306d201d7"],securityGroups=["sg-099a8ada8046cb487"],assignPublicIp="ENABLED"}'\'' \
		  --task-definition $$GEN_TASK_DEF \
		  --count 1 \
		  --region us-east-1 \
		  --platform-version LATEST; \
	'

	@echo "🎬 Running ECS processor task..."
	@bash -c '\
		PROC_FAMILY="BatchProcessorStackBatchProcessorTaskDefE076FDF4"; \
		PROC_TASK_DEF=$$(aws ecs list-task-definitions \
			--family-prefix $$PROC_FAMILY \
			--sort DESC \
			--max-items 1 \
			--output text \
			--query "taskDefinitionArns[0]"); \
		echo "👉 Using processor task definition: $$PROC_TASK_DEF"; \
		aws ecs run-task \
		  --cluster default \
		  --launch-type FARGATE \
		  --network-configuration '\''awsvpcConfiguration={subnets=["subnet-00939e65306d201d7"],securityGroups=["sg-099a8ada8046cb487"],assignPublicIp="ENABLED"}'\'' \
		  --task-definition $$PROC_TASK_DEF \
		  --count 1 \
		  --region us-east-1 \
		  --platform-version LATEST; \
	'

# === One-liner to build, push, deploy, and run ===
cloud: publish deploy execute

# === Clean build artifacts ===
clean:
	rm -f json2parquet_binary $(TAG_FILE)


