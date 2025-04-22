#!/bin/bash

echo "Resetting files..."
cp -f backups/backup_txt.txt samples/txt.txt
cp -f backups/backup_sql.sql samples/sql.sql
cp -f backups/backup_xml.xml samples/xml.xml
cp -f backups/backup_json.json samples/json.json
cp -f backups/backup_css.css samples/css.css
echo "Done!"

