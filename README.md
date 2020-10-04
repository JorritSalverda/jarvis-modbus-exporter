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

Modbus documentation for various devices:

* Sunny TriPower 8.0 - https://files.sma.de/downloads/MODBUS-HTML_STP8.0-10.0-3AV-40_GG10_V10.zip
* Alpha Innotec SWCV 92 K3 - https://www.nathan.nl/wp-content/uploads/2018/06/83055700cNL_alpha_connect.pdf