## Installation

To install this application using Helm run the following commands: 

```bash
helm repo add jorritsalverda https://helm.jorritsalverda.com
kubectl create namespace jarvis-modbus-exporter

helm upgrade \
  jarvis-modbus-exporter-alpha-innotec \
  jorritsalverda/jarvis-modbus-exporter \
  --install \
  --namespace jarvis-modbus-exporter-alpha-innotec \
  --set secret.gcpServiceAccountKeyfile='{abc: blabla}' \
  --wait
```
