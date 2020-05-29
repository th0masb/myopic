pipeline {
    agent any
    stages {
        stage("Setup build environment") {
            steps {
                // Bit of a hack to set the build name
                script {
                    currentBuild.displayName = "#${currentBuild.number} ${BRANCH}"
                }
                sh "rustup update"
            }
        }

        stage("Setup source code") {
            steps {
                echo "Cloning myopic into ${pwd()}"
                git credentialsId: "maumay-github-ssh", url: "git@github.com:maumay/myopic.git"
                sh 'ls -la'
            }
        }
        
        stage("Build") {
            steps {
                dir(path: "./${PROJECT}") {
                    echo pwd()
                    sh 'cargo check'
                    sh 'cargo test --release'
                }
            }
        }
    }
}
