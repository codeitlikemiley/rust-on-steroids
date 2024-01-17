FROM codeitlikemiley/ros:latest AS development

ARG USERNAME=vscode

COPY .bash_aliases /home/vscode/.bash_aliases
RUN echo 'eval "$(starship init bash)"' >> /home/vscode/.bashrc
RUN echo 'git config core.hooksPath /workspace/.devcontainer/.githooks' >> ~/.bashrc
# Uncomment
# RUN git config --global user.email "codeitlikemiley@gmail.com"
# RUN  git config --global user.name "Uriah Galang"

# all the volumes configured in the docker-compose.yml
RUN sudo mkdir -p /workspace/target && sudo chown $USERNAME:$USERNAME /workspace/target
RUN git config --global --add safe.directory /workspace
#RUN sudo mkdir -p /workspace/crates/asset-pipeline/node_modules && sudo chown $USERNAME:$USERNAME /workspace/crates/asset-pipeline/node_modules
