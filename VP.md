apt update -y 
apt install -y curl
apt install -y socat

curl https://get.acme.sh | sh
~/.acme.sh/acme.sh --register-account -m bradenzhang009@outlook.com
iptables -I INPUT -p tcp --dport 80 -j ACCEPT
~/.acme.sh/acme.sh  --issue -d am.orianfee.xyz   --standalone

~/.acme.sh/acme.sh --installcert -d am.orianfee.xyz --key-file /root/private.key --fullchain-file /root/cert.crt

bash <(curl -Ls https://raw.githubusercontent.com/vaxilu/x-ui/master/install.sh)

iptables -I INPUT -p tcp --dport 443 -j ACCEPT
iptables -I INPUT -p tcp --dport 2022 -j ACCEPT
iptables -I INPUT -p tcp --dport 2019 -j ACCEPT
iptables -I INPUT -p tcp --dport 2077 -j ACCEPT
