pipeline {
    agent none

    stages {
        stage('Doc') {
            parallel {
                stage('Cargo doc') {
                    agent {
                        label "rust"
                    }
                    steps {
                        git 'https://devcentral.nasqueron.org/source/docker-registry-api.git'

                        sh '''
                        cargo build
                        cargo doc || cargo doc --no-deps
                        cd target/doc
                        tar czf ../doc-rust.tar.gz *
                        '''
                    }
                    post {
                        success {
                            archiveArtifacts artifacts: 'target/doc-rust.tar.gz'
                        }
                    }
                }

                stage('OpenAPI doc') {
                    agent {
                        label "node"
                    }
                    steps {
                        git 'https://devcentral.nasqueron.org/source/docker-registry-api.git'

                        sh '''
                        mkdir -p target
                        cd target

                        spectacle ../nasqueron-api-docker-registry.spec.yaml --logo-file "https://assets.nasqueron.org/logos/logo-main-133px.png"
                        cd public

                        mkdir -p images
                        cd images
                        wget https://assets.nasqueron.org/logos/logo-main-133px.png
                        cd ..
                        sed -i "s/75px/131px/g" stylesheets/*.css

                        tar czf ../doc-openapi.tar.gz *
                        '''
                    }
                    post {
                        success {
                            archiveArtifacts artifacts: 'target/doc-openapi.tar.gz'
                        }
                    }
                }
            }
        }

        stage('Publish') {
            agent any
            steps {
                sshPublisher(
                    failOnError: true,
                    publishers: [
                        sshPublisherDesc(
                            configName: 'ysul-deploy',
                            transfers: [
                                sshTransfer(
                                    execCommand: "deploy-docker-registry-api-doc ${env.BUILD_NUMBER}"
                                )
                            ]
                        )
                    ]
                )
            }
        }
    }
}
