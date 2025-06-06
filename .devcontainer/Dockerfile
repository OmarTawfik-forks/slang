############################################################################################
#                                                                                          #
#                                                                                          #
#     !!!!!!                KEEP THIS IMAGE AS MINIMAL AS POSSIBLE              !!!!!!     #
#                                                                                          #
#                                                                                          #
#     This image should be kept as minimal as possible; only installing the bare           #
#     essentials needed to run the repository, or fetch other tools.                       #
#                                                                                          #
#     Any dependency that is used to build/test this project should be installed and       #
#     configured from within the project that needs it (using Hermit).                     #
#                                                                                          #
#     1) Smaller container size means we can fetch and create it quicker.                  #
#     2) Different projects can require different environments/dependencies.               #
#     3) Maximum portability, for when we start supporting Windows/MacOS builds.           #
#     4) Makes it easier to work on this repository outside the container if needed.       #
#                                                                                          #
#                                                                                          #
############################################################################################

# Base Container

# _SLANG_DEV_CONTAINER_BASE_IMAGE_ (keep in sync)
FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04

RUN sudo apt-get update \
  && sudo apt-get install --yes "pkg-config" \
  && rm -rf /var/lib/apt/lists/*

# Setup default Bash and Zsh for local development inside the image:
COPY ./scripts/ /home/vscode/

# Users that mount their own dot files are responsible for sourcing Slang's `.rc` files themselves:
RUN echo "source /home/vscode/.bashrc_devcontainer" >> /home/vscode/.bashrc
RUN echo "source /home/vscode/.zshrc_devcontainer" >> /home/vscode/.zshrc
