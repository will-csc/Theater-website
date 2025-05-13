import smtplib
import sys
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText

def send_reset_email(receiver_email,retrieveCode):
    sender_email = "semestretrabalhofecaf@gmail.com"  # E-mail remetente
    password = "upzg yebt cdrg sgzz"  # Senha do e-mail remetente
    
    # Criando a mensagem do e-mail
    msg = MIMEMultipart()
    msg['From'] = sender_email
    msg['To'] = receiver_email
    msg['Subject'] = "Seu código de recuperação - Cine"
    
    body = "Seu código de recuperação de conta ---> " + retrieveCode

    msg.attach(MIMEText(body, 'plain'))

    # Conectando ao servidor SMTP do Gmail
    try:
        # Conexão com o servidor SMTP do Gmail
        server = smtplib.SMTP('smtp.gmail.com', 587)
        server.starttls()  # Criptografia para segurança
        # Login no servidor SMTP
        server.login(sender_email, password)

        # Enviar o e-mail
        text = msg.as_string()
        server.sendmail(sender_email, receiver_email, text)

        # Fechar a conexão com o servidor
        server.quit()
    
    except Exception as e:
        print(f"Ocorreu um erro ao enviar o e-mail: {e}")


receiver_email = sys.argv[1]
retrivecode = sys.argv[2]

send_reset_email(receiver_email, retrivecode)
