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

-- =======================
-- Multiple-Selection Part
-- =======================

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

-- ===============
-- Completion Part
-- ===============

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

