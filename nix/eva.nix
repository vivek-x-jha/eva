# SPDX-FileCopyrightText: 2024 Christina Sørensen
# SPDX-License-Identifier: EUPL-1.2
{
  pkgs,
  naersk',
  buildInputs,
  ...
}:

naersk'.buildPackage rec {
  pname = "eva";
  version = "git";

  src = ../.;
  doCheck = true;

  inherit buildInputs;
  nativeBuildInputs = with pkgs; [
    cmake
    pkg-config
    installShellFiles
    pandoc
  ];

  buildNoDefaultFeatures = true;
  buildFeatures = "git";

  postInstall = ''
    for page in eva.1 eva_colors.5 eva_colors-explanation.5; do
      sed "s/\$version/${version}/g" "man/$page.md" |
        pandoc --standalone -f markdown -t man >"man/$page"
    done
    installManPage man/eva.1 man/eva_colors.5 man/eva_colors-explanation.5
    installShellCompletion \
      --bash completions/bash/eva \
      --fish completions/fish/eva.fish \
      --zsh completions/zsh/_eva
  '';

  meta = with pkgs.lib; {
    description = "A personal fork of eza, a modern replacement for ls";
    longDescription = ''
      eva is a personal fork of eza and a modern replacement for ls. It uses
      colours for information by default, helping you distinguish between many
      types of files, such as whether you are the owner, or in the owning group.
      It also has extra features not present in the original ls, such as viewing
      the Git status for a directory, or recursing into directories with a tree
      view. eva is written in Rust, so it’s small, fast, and portable.
    '';
    homepage = "https://github.com/vivek-x-jha/eva";
    license = licenses.mit;
    mainProgram = "eva";
    maintainers = with maintainers; [ cafkafk ];
  };
}
