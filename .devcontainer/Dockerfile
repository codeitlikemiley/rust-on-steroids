FROM codeitlikemiley/ros:latest AS app

ARG USERNAME=vscode

COPY .bash_aliases /home/vscode/.bash_aliases
RUN echo 'eval "$(starship init bash)"' >> /home/vscode/.bashrc

# all the volumes configured in the docker-compose.yml
RUN sudo mkdir -p /workspace/target && sudo chown $USERNAME:$USERNAME /workspace/target
#RUN sudo mkdir -p /workspace/crates/asset-pipeline/node_modules && sudo chown $USERNAME:$USERNAME /workspace/crates/asset-pipeline/node_modules
