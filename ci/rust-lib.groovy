
node {
    stage("Update toolchain") {
        sh "rustup update"
    }

    stage("Clone Repository") {
        echo pwd()
    }

    stage("Apply linting") {
        println("I'm alive!!")
        println("I injected the variable ${BRANCH} from the remote call!")
    }
}
