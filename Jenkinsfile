pipeline {
    agent any
    stages {
	stage('Build frontend') {
		steps {
			sh "wasm-pack build ./frontend --target web --out-name wasm --out-dir frontend/static"
		}
	}
        stage('Build backend') {
            steps {
                sh "cargo build --bin cli_runner --release --target armv7-unknown-linux-gnueabihf"
           }
        }
        stage('Build firmware') {
            steps {
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
