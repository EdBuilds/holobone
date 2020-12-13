pipeline {
    agent any
    stages {
        stage('Build') {
            steps {
                sh "cargo build --bin cli_runner --release --target armv7-unknown-linux-gnueabihf"
		echo "compile pru fw"
		sh "pasm -b ./prufw/PRUDAC.p"
		sh "pasm -b ./prufw/PRUClock.p"
           }
        }
        stage('Test') {
            steps {
		echo "Placeholder for testing..."
            }
        }
        stage('Clippy') {
            steps {
                sh "cargo clippy --all"
            }
        }
        stage('Doc') {
            steps {
                sh "cargo doc"
            }
        }
    }
}
