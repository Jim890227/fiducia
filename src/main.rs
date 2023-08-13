mod route;
use route::Route;
mod clientcertificate;
use clientcertificate::ClientCertificate;
mod servercertificate;
use servercertificate::ServerCertificate;
use clap::Parser;
use std::io::Write;
use std::fs::File;
use std::env;
use std::io;
use std::process::Command;
use std::os::unix::process::CommandExt;
fn main()  
{
    let args: Vec<String> = env::args().collect();
    match args[1].as_str()
    {
        "route" =>
        {
            let route_args = Route::parse();
            let (profile, dns_name, backend_service_name, backend_service_port, backend_service_scheme)
            = route_args.get();
            let mut file1 = File::create("../../route_config.yaml").expect("Could not create file!");
            let certDuration = "12h";
            let certKeySize = "4096";
            let mut domainName = "";
            match profile
            {
                "external-domain-server-cert" =>
                {
                    domainName = "fiduciaedge.internal";
                },
                "external-domain-client-cert" =>
                {
                    domainName = "fiduciaedge.internal";
                },
                "internal-domain-server-cert" =>
                {
                    domainName = "cluster.local";
                },
                "internal-domain-client-cert" =>
                {
                    domainName = "cluster.local";
                },
                 _ => (),
             }
             let output1 = format!(
                     r#"ingress:
  certDuration: {}
  certKeySize: {}
  domainName: {}
  certIssuer: {}


  routes:
  - hostname: {}.{}
    backendServiceName: {}
    backendServicePort: {}
    backendServiceScheme: {}
    namespace: default

"#, certDuration,certKeySize, domainName, profile, dns_name, domainName, backend_service_name, backend_service_port, backend_service_scheme);
            file1.write_all(output1.as_bytes()).expect("write failed");
            let mut file2 = File::create("../../fecbctl.config").expect("Could not create file!");
            let output2 = format!(
                    r#"ingressRoute:
    profile:
        externalDomain:
            serverCert:
              certIssuerName: vault-issuer-servercert-external-domain
              domainName: fiduciaedge.internal
            clientCert:
              certIssuerName: vault-issuer-clientcert-external-domain
              domainName: fiduciaedge.internal
        internalDomain:
            serverCert:
              certIssuerName: vault-issuer-servercert-internal-domain
              domainName: cluster.local
            clientCert:
              certIssuerName: vault-issuer-clientcert-internal-domain
              domainName: cluster.local"#);
            file2.write_all(output2.as_bytes()).expect("write failed");
            Command::new("/bin/sh").args(["-c","cat ../../route_config.yaml ../../fecbctl.config > ../../config.yaml"])
            .spawn().expect("spawn failure");
            let mut file3 = File::create("../../ingressroute.yaml.j2").expect("Could not create file!");
            let output3 = format!(
                r#"{{% for item in ingress.routes %}}
---
apiVersion: cert-manager.io/v1
kind: Certificate
metadata:
  name: ingress-{{{{ item.hostname | replace('.','-')  }}}}
  namespace: {{{{ item.namespace }}}}
spec:
  secretName: ingress-{{{{ item.hostname | replace('.','-') }}}}
  duration: {{{{ ingress.certDuration }}}}
  issuerRef:
    name: {{{{ ingress.certIssuer }}}}
    kind: ClusterIssuer
  commonName: {{{{ item.hostname }}}}
  dnsNames:
  - {{{{ item.hostname }}}}
  privateKey:
    algorithm: RSA
    size: {{{{ ingress.certKeySize }}}}

---

{{% if item.backendServiceScheme == 'https' %}}
apiVersion: traefik.containo.us/v1alpha1
kind: ServersTransport
metadata:
  name: {{{{ item.backendServiceName }}}}-server-transport
  namespace: {{{{ item.namespace }}}}


spec:
  serverName: {{{{ item.backendServiceName }}}}
  insecureSkipVerify: true
  forwardingTimeouts:
    dialTimeout: 42s
    responseHeaderTimeout: 42s
    idleConnTimeout: 42s
{{% endif %}}


---


apiVersion: traefik.containo.us/v1alpha1
kind: IngressRoute
metadata:
  name: ingress-{{{{ item.hostname | replace('.','-') }}}}
  namespace: {{{{ item.namespace }}}}
  labels:
    app.kubernetes.io/name: traefik
    app.kubernetes.io/instance: traefik
spec:
  entryPoints:
    - websecure
  routes:
  - match: Host("{{{{ item.hostname }}}}")
    kind: Rule
    services:
    - name: {{{{ item.backendServiceName }}}}
      port: {{{{ item.backendServicePort }}}}
      scheme: {{{{ item. backendServiceScheme }}}}
      passHostHeader: true
      {{% if item.backendServiceScheme == 'https' %}}serversTransport: {{{{ item.backendServiceName }}}}-server-transport{{% endif %}}
  tls:
    secretName: ingress-{{{{ item.hostname | replace('.','-') }}}}
    domains:
    - main: {{{{ ingress.domainName }}}}
      sans:
      - {{{{ item.hostname }}}}
    # mtls
    # options:
    #   name: traefik-client-auth
    #   namespace: $NAMESPACE_TLS_OPTION


{{% endfor %}}"#);
            file3.write_all(output3.as_bytes()).expect("write failed");
            Command::new("j2").arg("-f").arg("yaml")
            .args(["../../ingressroute.yaml.j2","../../config.yaml"])
            .stdout(File::create("../../ingressroute.yaml").unwrap())
            .spawn();
            Command::new("kubectl").arg("apply").arg("-f").arg("../../ingressroute.yaml").exec();
        }   
        "cert" =>
        {
            match args[2].as_str()
            {
                "client" =>
                {
                    let certificate_args = ClientCertificate::parse();
                    let (profile, common_name, duration, key_size)
                    = certificate_args.get();
                    let mut file1 = File::create("../../client_config.yaml").expect("Could not create file!");
                    let output1 = format!(
                        r#"certificate:
    profile: {}
    commonName: {}
    duration: {}
    keysize: {}
    namespace: default

"#,profile,common_name,duration,key_size);
                    file1.write_all(output1.as_bytes()).expect("write failed");
                    let mut file2 = File::create("../../fecbctl.config").expect("Could not create file!");
                    let output2 = format!(
                            r#"ingressRoute:
    profile:
        externalDomain:
            serverCert:
                certIssuerName: vault-issuer-servercert-external-domain
                domainName: fiduciaedge.internal
            clientCert:
                certIssuerName: vault-issuer-clientcert-external-domain
                domainName: fiduciaedge.internal
        internalDomain:
            serverCert:
                certIssuerName: vault-issuer-servercert-internal-domain
                domainName: cluster.local
            clientCert:
                certIssuerName: vault-issuer-clientcert-internal-domain
                domainName: cluster.local"#);
                    file2.write_all(output2.as_bytes()).expect("write failed");
                    Command::new("/bin/sh").args(["-c","cat ../../client_config.yaml ../../fecbctl.config > ../../config.yaml"])
                    .spawn().expect("spawn failure");
                    let mut file3 = File::create("../../certificate.yaml.j2").expect("Could not create file!");
                    let output3 = format!(
                    r#"apiVersion: cert-manager.io/v1
kind: Certificate
metadata:
  name: {{{{ certificate.commonName | replace('.','-') | replace('@','-') }}}}
  namespace: {{{{ certificate.namespace }}}}
spec:
  secretName: ingress-{{{{ certificate.commonName | replace('.','-') | replace('@','-') }}}}
  duration: {{{{ certificate.duration }}}}
  issuerRef:
    name: {{%- if certificate.profile == 'external-domain-server-cert' %}} ingressRoute.profile.externalDomain.serverCert.certIssuerName
          {{%- elif certificate.profile == 'external-domain-client-cert' %}} ingressRoute.profile.externalDomain.clientCert.certIssuerName
          {{%- elif certificate.profile == 'internal-domain-server-cert' %}} ingressRoute.profile.internalDomain.serverCert.certIssuerName
          {{%- elif certificate.profile == 'internal-domain-client-cert' %}} ingressRoute.profile.internalDomain.clientCert.certIssuerName
          {{%- endif %}}
    kind: ClusterIssuer
  commonName: {{{{ certificate.commonName }}}}
  dnsNames:
  - {{{{ certificate.commonName }}}}
  privateKey:
    algorithm: RSA
    size: {{{{ certificate.keysize }}}}
  "#);
                file3.write_all(output3.as_bytes()).expect("write failed");    
                Command::new("j2").arg("-f").arg("yaml")
                .args(["../../certificate.yaml.j2","../../config.yaml"])
                .stdout(File::create("../../certificate.yaml").unwrap())
                .spawn();
                Command::new("kubectl").arg("apply").arg("-f").arg("../../certificate.yaml").exec();
                
                },
                "server" =>
                {
                    let certificate_args = ServerCertificate::parse();
                    let (profile, common_name, duration, key_size)
                    = certificate_args.get();
                    let mut file1 = File::create("../../server_config.yaml").expect("Could not create file!");
                    let output1 = format!(
                        r#"certificate:
    profile: {}
    commonName: {}
    duration: {}
    keysize: {}
    namespace: default

"#,profile,common_name,duration,key_size);
                    file1.write_all(output1.as_bytes()).expect("write failed");
                    let mut file2 = File::create("../../fecbctl.config").expect("Could not create file!");
                    let output2 = format!(
                            r#"ingressRoute:
    profile:
        externalDomain:
            serverCert:
                certIssuerName: vault-issuer-servercert-external-domain
                domainName: fiduciaedge.internal
            clientCert:
                certIssuerName: vault-issuer-clientcert-external-domain
                domainName: fiduciaedge.internal
        internalDomain:
            serverCert:
                certIssuerName: vault-issuer-servercert-internal-domain
                domainName: cluster.local
            clientCert:
                certIssuerName: vault-issuer-clientcert-internal-domain
                domainName: cluster.local"#);
                    file2.write_all(output2.as_bytes()).expect("write failed");
                    Command::new("/bin/sh").args(["-c","cat ../../server_config.yaml ../../fecbctl.config > ../../config.yaml"])
                    .spawn().expect("spawn failure");
                    let mut file3 = File::create("../../certificate.yaml.j2").expect("Could not create file!");
                    let output3 = format!(
                    r#"apiVersion: cert-manager.io/v1
kind: Certificate
metadata:
  name: {{{{ certificate.commonName | replace('.','-')  }}}}
  namespace: {{{{ certificate.namespace }}}}
spec:
  secretName: ingress-{{{{ certificate.commonName | replace('.','-') }}}}
  duration: {{{{ certificate.duration }}}}
  issuerRef:
    name: {{%- if certificate.profile == 'external-domain-server-cert' %}} ingressRoute.profile.externalDomain.serverCert.certIssuerName
          {{%- elif certificate.profile == 'external-domain-client-cert' %}} ingressRoute.profile.externalDomain.clientCert.certIssuerName
          {{%- elif certificate.profile == 'internal-domain-server-cert' %}} ingressRoute.profile.internalDomain.serverCert.certIssuerName
          {{%- elif certificate.profile == 'internal-domain-client-cert' %}} ingressRoute.profile.internalDomain.clientCert.certIssuerName
          {{%- endif %}}
    kind: ClusterIssuer
  commonName: {{{{ certificate.commonName }}}}
  dnsNames:
  - {{{{ certificate.commonName }}}}
  privateKey:
    algorithm: RSA
    size: {{{{ certificate.keysize }}}}
  "#);
                file3.write_all(output3.as_bytes()).expect("write failed");    
                Command::new("j2").arg("-f").arg("yaml")
                .args(["../../certificate.yaml.j2","../../config.yaml"])
                .stdout(File::create("../../certificate.yaml").unwrap())
                .spawn();
                Command::new("kubectl").arg("apply").arg("-f").arg("../../certificate.yaml").exec();
                },
                _ => (),
            }
  
        },
        _ => (),
    }
        
    
}

