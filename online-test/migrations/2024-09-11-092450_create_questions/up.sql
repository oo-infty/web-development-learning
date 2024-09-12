CREATE TABLE questions (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  kind INTEGER NOT NULL,
  content TEXT NOT NULL,
  option0 TEXT,
  option1 TEXT,
  option2 TEXT,
  option3 TEXT,
  answer TEXT NOT NULL
);

-- =====================
-- Single-Selection Part
-- =====================

-- CLI Operation

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'Which command is used to list all currently running processes with detailed information, including the PID?',
    '<code>ps aux</code>',
    '<code>top</code>',
    '<code>pstree</code>',
    '<code>htop</code>',
    '0'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'Which command would you use to search for the string "error" within the system log file located at <code>/var/log/syslog</code>?',
    '<code>find /var/log/syslog -name "error"</code>',
    '<code>search "error" /var/log/syslog</code>',
    '<code>grep "error" /var/log/syslog</code>',
    '<code>cat /var/log/syslog | grep "error"</code>',
    '2'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'Which command allows you to display the disk usage of files and directories in a human-readable format?',
    '<code>df -h</code>',
    '<code>du -h</code>',
    '<code>ls -lh</code>',
    '<code>diskusage -h</code>',
    '1'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'Which command is used to change the ownership of a file to a specific user?',
    '<code>chown user filename</code>',
    '<code>chmod user filename</code>',
    '<code>chgrp user filename</code>',
    '<code>chown filename user</code>',
    '0'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'What is the correct command to check the available and used memory on your system?',
    '<code>memory -a</code>',
    '<code>top -m</code>',
    '<code>meminfo</code>',
    '<code>free -m</code>',
    '3'
);

-- Process & Service Management

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'Which command allows you to view and manage the processes that are currently using the most CPU time?',
    '<code>ps aux --sort=-%cpu</code>',
    '<code>htop</code>',
    '<code>top -p</code>',
    '<code>top -o %CPU</code>',
    '3'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'What is the purpose of the <code>systemctl daemon-reload</code> command?',
    'It restarts all active services.',
    'It reloads the systemd manager configuration.',
    'It updates the kernel modules.',
    'It refreshes user environment variables.',
    '1'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'How can you stop a service and ensure it is not started automatically at boot time?',
    '<code>systemctl stop <service> && systemctl disable <service></code>',
    '<code>pkill -f <service> && chkconfig <service> off</code>',
    '<code>service <service> stop && update-rc.d <service> remove</code>',
    '<code>systemctl kill <service> && systemctl mask <service></code>',
    '0'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'Which command would you use to check the status of a specific service managed by systemd?',
    '<code>ps -ef | grep <service></code>',
    '<code>systemctl status <service></code>',
    '<code>systemctl list-units --type=service</code>',
    '<code>service <service> status</code>',
    '1'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'What does the <code>kill -9</code> command do?',
    'It pauses a process.',
    'It forcefully terminates a process without allowing it to clean up.',
    'It sends a termination signal to gracefully stop a process.',
    'It restarts a process.',
    '1'
);

-- Disk Management

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'What does the <code>lvcreate</code> command do in Linux Logical Volume Manager (LVM)?',
    'It creates a new physical volume.',
    'It creates a new logical volume.',
    'It creates a new volume group.',
    'It removes an existing logical volume.',
    '1'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'Which command would you use to check the current disk usage of all mounted filesystems in Linux?',
    '<code>df -h</code>',
    '<code>du -sh</code>',
    '<code>lsblk</code>',
    '<code>fdisk -l</code>',
    '0'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'How can you display detailed information about all disks and partitions in Linux?',
    '<code>lsblk -f</code>',
    '<code>blkid</code>',
    '<code>parted -l</code>',
    '<code>fdisk -l</code>',
    '3'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'What is the purpose of the <code>resize2fs</code> command?',
    'To resize a disk partition.',
    'To resize a file system.',
    'To check for disk errors.',
    'To format a file system.',
    '1'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'Which of the following commands would you use to add a new disk to an existing volume group in LVM?',
    '<code>lvextend</code>',
    '<code>pvcreate</code>',
    '<code>vgextend</code>',
    '<code>vgcreate</code>',
    '2'
);

-- System & Software Deployment

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'What is the purpose of the <code>chroot</code> command in Linux?',
    'It changes the root directory of a process and its children to a specified path.',
    'It changes the permissions of a file or directory.',
    'It moves the kernel to a different location on disk.',
    'It creates a new user account.',
    '0'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'When configuring a network interface using <code>ifconfig</code>, what does the <code>up</code> parameter do?',
    'It brings the network interface down, disabling it.',
    'It restarts the network interface.',
    'It brings the network interface up, enabling it.',
    'It updates the IP address of the network interface.',
    '2'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'Which command is used to view and modify the current network routing table?',
    'The <code>netstat</code> command.',
    'The <code>ip route</code> command.',
    'The <code>route</code> command.',
    'The <code>ifconfig</code> command.',
    '1'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'What does the <code>rsync -avz</code> command do?',
    'It creates a backup of files with verification and zero compression.',
    'It moves files to a new directory while preserving file attributes.',
    'It recursively copies files while preserving symbolic links.',
    'It synchronizes files and directories between two locations with archive mode, verbose output, and compression.',
    '3'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'What is the role of the <code>/etc/fstab</code> file in Linux systems?',
    'It lists all user accounts and their passwords.',
    'It contains configuration details for the system’s boot loader.',
    'It defines how disk partitions, filesystems, and remote filesystems are mounted at boot time.',
    'It stores kernel parameters used during boot.',
    '2'
);

-- VM & Container Operation

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'Which command is used to list all currently running Docker containers?',
    '<code>docker list</code>',
    '<code>docker ps</code>',
    '<code>docker containers</code>',
    '<code>docker ls</code>',
    '1'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'In a KVM-based virtual machine environment, which command provides information about the virtual machine’s current state?',
    '<code>virt-manager status</code>',
    '<code>kvm status</code>',
    '<code>vmctl info</code>',
    '<code>virsh list</code>',
    '3'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'What is the purpose of the <code>docker-compose.yml</code> file?',
    'It defines Docker images and their build context.',
    'It specifies environment variables for Docker containers.',
    'It outlines the configuration for running multiple Docker containers as a service.',
    'It sets the logging options for Docker containers.',
    '2'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'Which command would you use to increase the memory allocated to a VirtualBox VM?',
    '<code>VBoxManage modifyvm "VM_NAME" --memory SIZE</code>',
    '<code>VBoxManage setram "VM_NAME" SIZE</code>',
    '<code>VBoxManage changevm "VM_NAME" --memory SIZE</code>',
    '<code>VBoxManage resize "VM_NAME" --memory SIZE</code>',
    '0'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    0,
    'In a Docker container, which command is used to view the logs of a specific container?',
    '<code>docker view CONTAINER_ID</code>',
    '<code>docker logs CONTAINER_ID</code>',
    '<code>docker status CONTAINER_ID</code>',
    '<code>docker inspect CONTAINER_ID</code>',
    '1'
);

-- =======================
-- Multiple-Selection Part
-- =======================

-- CLI Operation

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which commands can be used to check disk space usage on a Linux system?',
    '<code>lsblk</code>',
    '<code>df -h</code>',
    '<code>du -sh</code>',
    '<code>free -m</code>',
    '12'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which commands are useful for viewing and analyzing log files?',
    '<code>find</code>',
    '<code>less</code>',
    '<code>grep</code>',
    '<code>cat</code>',
    '123'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which commands can be used to monitor real-time system performance?',
    '<code>top</code>',
    '<code>htop</code>',
    '<code>grep</code>',
    '<code>vmstat</code>',
    '013'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which commands can be used to manage file permissions and ownership?',
    '<code>chmod</code>',
    '<code>ls -a</code>',
    '<code>chown</code>',
    '<code>chgrp</code>',
    '023'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which commands can be used to find files and directories on a Linux system?',
    '<code>ls</code>',
    '<code>find</code>',
    '<code>locate</code>',
    '<code>whereis</code>',
    '123'
);

-- Process & Service Management

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which commands can be used to check and manage currently running services on a systemd-based Linux system?',
    '<code>systemctl status</code>',
    '<code>service --status-all</code>',
    '<code>ps -ef</code>',
    '<code>systemctl list-units --type=service</code>',
    '03'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which commands are useful for viewing detailed information about processes?',
    '<code>ps aux</code>',
    '<code>free -h</code>',
    '<code>top -c</code>',
    '<code>htop</code>',
    '023'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which commands can be used to check which services are enabled to start at boot time?',
    '<code>chkconfig --list</code>',
    '<code>systemctl is-enabled <service></code>',
    '<code>service --list</code>',
    '<code>systemctl list-unit-files --type=service</code>',
    '13'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which commands can be used to modify the behavior of systemd services?',
    '<code>systemctl enable <service></code>',
    '<code>systemctl disable <service></code>',
    '<code>systemctl status <service></code>',
    '<code>systemctl restart <service></code>',
    '013'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which commands are commonly used to monitor network connections and statistics?',
    '<code>ping</code>',
    '<code>netstat -tuln</code>',
    '<code>ss -tuln</code>',
    '<code>ifconfig</code>',
    '123'
);

-- Disk Management

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which commands can be used to display information about disk partitions?',
    '<code>mount</code>',
    '<code>df -h</code>',
    '<code>fdisk -l</code>',
    '<code>lsblk</code>',
    '23'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'What commands can be used to create a new partition on a disk?',
    '<code>parted</code>',
    '<code>mkfs</code>',
    '<code>fdisk</code>',
    '<code>gparted</code>',
    '013'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which commands are used to check the file system for errors?',
    '<code>parted</code>',
    '<code>fsck</code>',
    '<code>chkdsk</code>',
    '<code>df</code>',
    '12'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which tools are used for managing Logical Volume Management (LVM) volumes?',
    '<code>lvcreate</code>',
    '<code>vgextend</code>',
    '<code>pvcreate</code>',
    '<code>fsck</code>',
    '023'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'What commands are used to resize a partition or volume?',
    '<code>resize2fs</code>',
    '<code>lvresize</code>',
    '<code>fdisk</code>',
    '<code>parted</code>',
    '013'
);

-- System & Software Deployment

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which commands are used to manage software packages on a Red Hat-based Linux distribution?',
    '<code>yum install</code>',
    '<code>rpm -i</code>',
    '<code>dnf update</code>',
    '<code>apt-get upgrade</code>',
    '012'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which files are typically edited to configure static IP addresses on a Debian-based system?',
    '<code>/etc/sysconfig/network-scripts/ifcfg-eth0</code>',
    '<code>/etc/network/interfaces</code>',
    '<code>/etc/resolv.conf</code>',
    '<code>/etc/netplan/*.yaml</code>',
    '13'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which files and directories are essential for configuring Apache HTTP server on a Linux system?',
    '<code>/etc/httpd/conf/httpd.conf</code>',
    '<code>/etc/apache2/apache2.conf</code>',
    '<code>/var/www/html/</code>',
    '<code>/etc/hosts</code>',
    '012'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which files and commands are relevant for configuring SSH access and security on a Linux system?',
    '<code>sshd</code>',
    '<code>/etc/ssh/sshd_config</code>',
    '<code>/etc/ssh/ssh_config</code>',
    '<code>ssh-keygen</code>',
    '123'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which files are important for configuring system locale settings on a Linux system?',
    '<code>/etc/locale.gen</code>',
    '<code>/etc/locale.conf</code>',
    '<code>/etc/environment</code>',
    '<code>/etc/default/locale</code>',
    '013'
);

-- VM & Container Operation

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which commands are used to manage Docker containers on a Linux system?',
    '<code>docker list</code>',
    '<code>docker ps</code>',
    '<code>docker container ls</code>',
    '<code>docker show</code>',
    '12'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'In a Linux environment, which commands are used to check the current status of virtual machines managed by KVM?',
    '<code>virsh list</code>',
    '<code>kvm status</code>',
    '<code>virsh status</code>',
    '<code>virt-manager status</code>',
    '01'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which files or directories are used for Docker configuration on a Linux system?',
    '<code>/var/lib/docker/config</code>',
    '<code>/etc/docker/config.json</code>',
    '<code>/usr/local/docker/config</code>',
    '<code>/etc/docker/daemon.json</code>',
    '13'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'When using Kubernetes, which commands are used to inspect the details of a pod?',
    '<code>kubectl show pod</code>',
    '<code>kubectl inspect pod</code>',
    '<code>kubectl get pod</code>',
    '<code>kubectl describe pod</code>',
    '23'
);

INSERT INTO questions (kind, content, option0, option1, option2, option3, answer)
VALUES (
    1,
    'Which files or directories are typically used for storing Kubernetes configuration files?',
    '<code>~/.kube/config</code>',
    '<code>/etc/kubernetes/admin.conf</code>',
    '<code>/var/lib/kubelet/config</code>',
    '<code>/etc/kubernetes/kubelet.conf</code>',
    '01'
);

-- ===============
-- Completion Part
-- ===============

-- CLI Operation

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'To list all the processes currently running on a Linux system, you can use the <code>ps</code> command with the <code>-ef</code> option. What command would you use to list all processes in a tree format?',
    'ps -ef --forest'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'To change the permissions of a file to allow the owner to read, write, and execute, but only allow others to read, you would use the <code>chmod</code> command. What numeric mode would you use for this permission setting?',
    'chmod 744'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'When managing disk space on a Linux system, you can use the <code>du</code> command to check the size of directories. What option with <code>du</code> will display the sizes in a human-readable format?',
    'du -h'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'To view the most recent entries in a log file located at <code>/var/log/syslog</code>, you can use the <code>tail</code> command. Which option will allow you to follow new entries in real-time?',
    'tail -f'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'In a situation where you need to search for a specific text string within files in a directory, which command can be used to recursively search through files for the text string "error"?',
    'grep -r "error"'
);

-- Process & Service Management

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'When configuring a service to restart automatically upon failure, which configuration directive should be added to the service unit file for systemd to achieve this?',
    'Restart=on-failure'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'To display the environment variables for a specific running process with PID 5678, which command should you use?',
    'cat /proc/5678/environ'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'If you want to view real-time log entries for a service managed by systemd, such as "mysql", which command will you use?',
    'journalctl -u mysql -f'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'To view the process hierarchy and find out which processes are child processes of PID 1000, which command should be used?',
    'pstree -p 1000'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'To change the priority of a running process with PID 7890 to a higher priority (lower nice value), which command should be used?',
    'renice -n -10 -p 7890'
);

-- Disk Management

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'When you want to extend a filesystem on a logical volume after resizing the logical volume, you should use the command <code>resize2fs</code>. Fill in the command to resize the filesystem on <code>/dev/vg_data/lv_home</code> to use all available space:',
    'resize2fs /dev/vg_data/lv_home'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'To create a new partition on a disk, the command <code>fdisk</code> can be used. To start partitioning the disk <code>/dev/sda</code>, which command should be issued?',
    'fdisk /dev/sda'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'To check and repair filesystem errors on the partition <code>/dev/sda1</code>, you can use the <code>fsck</code> command. What is the full command to perform a check and repair?',
    'fsck /dev/sda1'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'When creating a new logical volume, you need to first create a volume group. To create a volume group named <code>vg_data</code> from the physical volume <code>/dev/sdb1</code>, which command should be used?',
    'vgcreate vg_data /dev/sdb1'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'To resize an existing logical volume named <code>lv_home</code> in the volume group <code>vg_data</code> to 10GB, which command should be used?',
    'lvresize -L 10G /dev/vg_data/lv_home'
);

-- System & Software Deployment

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'To configure automatic updates for security patches on an Ubuntu system, which package should be installed and configured?',
    'unattended-upgrades'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'To check the current version of the installed <code>nginx</code> package on a Red Hat-based system, which command should be used?',
    'rpm -q nginx'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'To modify the network interface configuration file for <code>eth0</code> on a Linux system, which file should be edited?',
    '/etc/network/interfaces'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'To update the package list and upgrade all installed packages on a Debian-based system, which command sequence should be used?',
    'apt update && apt upgrade'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'To check the status of the firewall on a Linux system using <code>firewalld</code>, which command should be used?',
    'firewall-cmd --state'
);

-- VM & Container Operation

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'To list all Docker images on a system, which command should be used?',
    'docker images'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'In Kubernetes, to apply a configuration file named <code>deployment.yaml</code> to a cluster, which command should be used?',
    'kubectl apply -f deployment.yaml'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'To create a new virtual machine using the <code>virt-install</code> command, which option specifies the amount of RAM allocated to the VM?',
    '--ram'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'When using Docker Compose, to start up the services defined in the <code>docker-compose.yml</code> file, which command should be used?',
    'docker-compose up'
);

INSERT INTO questions (kind, content, answer)
VALUES (
    2,
    'To view the current resource usage of all running containers, including CPU and memory, which command should be used?',
    'docker stats'
);
