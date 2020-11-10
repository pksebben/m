import subprocess
# O to the N
subprocess.Popen(['/bin/sh', '-c', 'node n.js'], stdout=subprocess.PIPE)
